use std::collections::HashMap;
use std::path::{Prefix, PrefixComponent};
use crate::engine::bitboard::{BitBoard, BitPos, Color, Square};
use crate::engine::movegen::{Move, movegen};
use std::rand::Rng;
use crate::engine::bitboard::Kind::Undefined;

pub struct PrecomputedBitBoards {
    pub rook_boards: [BitBoard; 64],
    pub knight_boards: [BitBoard; 64],
    pub magic_numbers: [[u64; 64];2],
}

fn generate_magic_bitboards(precomputed: PrecomputedBitBoards) -> [[u64; 64]; 2] {

    const RBits: [u8; 64] = [
        12, 11, 11, 11, 11, 11, 11, 12,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        11, 10, 10, 10, 10, 10, 10, 11,
        12, 11, 11, 11, 11, 11, 11, 12
    ];

    const BBits: [u8; 64] = [
        6, 5, 5, 5, 5, 5, 5, 6,
        5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 9, 9, 7, 5, 5,
        5, 5, 7, 7, 7, 7, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5,
        6, 5, 5, 5, 5, 5, 5, 6
    ];

    const BitTable: [u64; 64] = [
        63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2,
        51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52,
        26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28,
        58, 20, 37, 17, 36, 8
    ];


    fn random_u64 () -> u64 {
        let mut rng = rand::XorShiftRng::new_unseeded();
        return rng.next_u64();
    }

    fn random_u64_fewbits () -> u64 {
        return random_uint64() & random_uint64() & random_uint64();
    }

    fn pop_1st_bit (bb: &mut u64) -> u64 {
        let b = *bb ^ (*bb - 1);
        let fold: u64 = ((b & 0xffffffff) ^ (b >> 32));
        *bb &= (*bb - 1);
        return BitTable[(fold * 0x783a9b23) >> 26];
    }

    fn index_to_uint(i: usize, n: u32, mut mask: u64) -> u64 {
        let (i, mut j) = (0u64, 0u64);
        let result = 0u64;
        while 1 {
            if i < bits {break}
            j = pop_1st_bit(&mut mask);
        }
        result
    }

    fn count_1s (b: u64) -> u64 {

    }

    fn transform(b: u8, magic: u64, bits: u64) -> u64 {
        return ((b * magic) >> (64 - bits)) as u64;
    }

    fn rmask(square: u8, precomputed: PrecomputedBitBoards) -> u64 {
        return precomputed.rook_boards[square].0
    }

    fn bmask (square: u8, precomputed: PrecomputedBitBoards) -> u64 {
        return precomputed.rook_boards[square].0
    }

    fn ratt (square: u8, block: u64) -> u64 {

    }

    fn batt (square: u8, block: u64) -> u64 {

    }

    fn find_magic(square: u8, BIT: u8, bishop: u8) -> u64 {
        let mask: u64 = 0;
        let mut a: [u64; 4096] = [0; 4096];
        let mut b: [u64; 4096] = [0; 4096];
        let mut used: [u64; 4096] = [0; 4096];
        let mut magic: u64 = 0;

        match bishop {
            0 => {bmask(sq)},
            1 => {rmask(sq)},
            _ => {}
        }

        let (i, mut j, k, n, fail) = (0u32, 0u64, 0u32, 0u32, 0u8);



        for i in 0..(1<<n) {
            b[i] = index_to_uint(i, n, mask);
            match bishop {
                0 => {
                    a[i] = batt(square, b[i]);
                },
                1 => {
                    a[i] = ratt(square, b[i]);
                },
                _ => {}
            }
        }

        'outer: for k in 0u64..100000000u64 {
            magic = random_u64_fewbits();
            if count_1s((mask * magic) & 0xFF00000000000000) < 6 {continue};
            for i in 0usize..4096usize {
                used[i] = 0;
            }
            let (i, mut fail) = (0u64, 0u64);
            while 1 {
                if !fail {
                    if i < (1 << n) {
                        break;
                    }
                }
                j = transform(b[i], magic, m);
                if used[j] == 0 {used[j] = a[i]}
                else if used[j] != a[i] {fail = 1};
            }
            if fail == 0 {return magic}
        }
        return 0u64
    }

    let mut magic_boards: [[u64; 64];2] = [[0; 64]; 2];

    for field in 0u8..64u8 {

    }

    return magic_boards
}

impl PrecomputedBitBoards {
    fn new() -> Self {
        Self {
            knight_boards: [
                BitBoard::from(132096),
                BitBoard::from(329728),
                BitBoard::from(659712),
                BitBoard::from(1319424),
                BitBoard::from(2638848),
                BitBoard::from(5277696),
                BitBoard::from(10489856),
                BitBoard::from(4202496),
                BitBoard::from(33816580),
                BitBoard::from(84410376),
                BitBoard::from(168886289),
                BitBoard::from(337772578),
                BitBoard::from(675545156),
                BitBoard::from(1351090312),
                BitBoard::from(2685403152),
                BitBoard::from(1075839008),
                BitBoard::from(8657044482),
                BitBoard::from(21609056261),
                BitBoard::from(43234889994),
                BitBoard::from(86469779988),
                BitBoard::from(172939559976),
                BitBoard::from(345879119952),
                BitBoard::from(687463207072),
                BitBoard::from(275414786112),
                BitBoard::from(2216203387392),
                BitBoard::from(5531918402816),
                BitBoard::from(11068131838464),
                BitBoard::from(22136263676928),
                BitBoard::from(44272527353856),
                BitBoard::from(88545054707712),
                BitBoard::from(175990581010432),
                BitBoard::from(70506185244672),
                BitBoard::from(567348067172352),
                BitBoard::from(1416171111120896),
                BitBoard::from(2833441750646784),
                BitBoard::from(5666883501293568),
                BitBoard::from(11333767002587136),
                BitBoard::from(22667534005174272),
                BitBoard::from(45053588738670592),
                BitBoard::from(18049583422636032),
                BitBoard::from(145241105196122112),
                BitBoard::from(362539804446949376),
                BitBoard::from(725361088165576704),
                BitBoard::from(1450722176331153408),
                BitBoard::from(2901444352662306816),
                BitBoard::from(5802888705324613632),
                BitBoard::from(11533718717099671552),
                BitBoard::from(4620693356194824192),
                BitBoard::from(288234782788157440),
                BitBoard::from(576469569871282176),
                BitBoard::from(1224997833292120064),
                BitBoard::from(2449995666584240128),
                BitBoard::from(4899991333168480256),
                BitBoard::from(9799982666336960512),
                BitBoard::from(1152939783987658752),
                BitBoard::from(2305878468463689728),
                BitBoard::from(1128098930098176),
                BitBoard::from(2257297371824128),
                BitBoard::from(4796069720358912),
                BitBoard::from(9592139440717824),
                BitBoard::from(19184278881435648),
                BitBoard::from(38368557762871296),
                BitBoard::from(4679521487814656),
                BitBoard::from(9077567998918656),
            ],
            rook_boards: [
                BitBoard::from(282578800148862),
                BitBoard::from(565157600297596),
                BitBoard::from(1130315200595066),
                BitBoard::from(2260630401190006),
                BitBoard::from(4521260802379886),
                BitBoard::from(9042521604759646),
                BitBoard::from(18085043209519166),
                BitBoard::from(36170086419038334),
                BitBoard::from(282578800180736),
                BitBoard::from(565157600328704),
                BitBoard::from(1130315200625152),
                BitBoard::from(2260630401218048),
                BitBoard::from(4521260802403840),
                BitBoard::from(9042521604775424),
                BitBoard::from(18085043209518592),
                BitBoard::from(36170086419037696),
                BitBoard::from(282578808340736),
                BitBoard::from(565157608292864),
                BitBoard::from(1130315208328192),
                BitBoard::from(2260630408398848),
                BitBoard::from(4521260808540160),
                BitBoard::from(9042521608822784),
                BitBoard::from(18085043209388032),
                BitBoard::from(36170086418907136),
                BitBoard::from(282580897300736),
                BitBoard::from(565159647117824),
                BitBoard::from(1130317180306432),
                BitBoard::from(2260632246683648),
                BitBoard::from(4521262379438080),
                BitBoard::from(9042522644946944),
                BitBoard::from(18085043175964672),
                BitBoard::from(36170086385483776),
                BitBoard::from(283115671060736),
                BitBoard::from(565681586307584),
                BitBoard::from(1130822006735872),
                BitBoard::from(2261102847592448),
                BitBoard::from(4521664529305600),
                BitBoard::from(9042787892731904),
                BitBoard::from(18085034619584512),
                BitBoard::from(36170077829103616),
                BitBoard::from(420017753620736),
                BitBoard::from(699298018886144),
                BitBoard::from(1260057572672512),
                BitBoard::from(2381576680245248),
                BitBoard::from(4624614895390720),
                BitBoard::from(9110691325681664),
                BitBoard::from(18082844186263552),
                BitBoard::from(36167887395782656),
                BitBoard::from(35466950888980736),
                BitBoard::from(34905104758997504),
                BitBoard::from(34344362452452352),
                BitBoard::from(33222877839362048),
                BitBoard::from(30979908613181440),
                BitBoard::from(26493970160820224),
                BitBoard::from(17522093256097792),
                BitBoard::from(35607136465616896),
                BitBoard::from(9079539427579068672),
                BitBoard::from(8935706818303361536),
                BitBoard::from(8792156787827803136),
                BitBoard::from(8505056726876686336),
                BitBoard::from(7930856604974452736),
                BitBoard::from(6782456361169985536),
                BitBoard::from(4485655873561051136),
                BitBoard::from(9115426935197958144),
            ],
            magic_numbers: [[0; 64]; 2],
        }
    }
}

pub struct Game {
    side_to_move: Color,
    precomputed: PrecomputedBitBoards
}

impl Game {
    pub fn new() -> Self {
        Self {
            side_to_move: Color::White,
            precomputed: PrecomputedBitBoards::new(),
        }
    }

    pub fn start(&mut self, fen: &str) {
        self.precomputed.magic_numbers = generate_magic_bitboards();

        let mut board = BitPos::from_fen(fen);
        board.print();

        let moves: Vec<Move> = movegen(&mut board, Color::White as u8, &self.precomputed);

        for pos_move in moves {
            pos_move.print();

        }

        // for i in 0..30 {
        //     movegen(board, Color::White as u8, self.knight_boards);

        // }
        board.attack_white.print_index();
    }
}