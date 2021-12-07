fn calculate_fuel(start: u32, end: u32) -> u32 {
    let distance = (start as i32 - end as i32).abs() as u32;
    (1 + distance) * distance / 2
}

fn simulate_move(crabs: &[u32], position: u32) -> u32 {
    crabs
        .iter()
        .map(|current_pos| calculate_fuel(*current_pos, position))
        .sum()
}

fn main() {
    let input = [
        1101, 1, 29, 67, 1102, 0, 1, 65, 1008, 65, 35, 66, 1005, 66, 28, 1, 67, 65, 20, 4, 0, 1001,
        65, 1, 65, 1106, 0, 8, 99, 35, 67, 101, 99, 105, 32, 110, 39, 101, 115, 116, 32, 112, 97,
        115, 32, 117, 110, 101, 32, 105, 110, 116, 99, 111, 100, 101, 32, 112, 114, 111, 103, 114,
        97, 109, 10, 209, 573, 1277, 704, 518, 276, 196, 62, 1226, 170, 58, 1450, 101, 65, 99, 435,
        986, 1437, 1570, 35, 354, 247, 110, 105, 139, 1209, 23, 1074, 339, 69, 483, 21, 33, 323,
        1348, 111, 2, 270, 1239, 316, 529, 1680, 1056, 1960, 257, 1009, 1073, 59, 425, 1181, 198,
        31, 299, 771, 53, 817, 728, 931, 72, 517, 39, 279, 304, 401, 1271, 533, 1551, 133, 297,
        162, 902, 370, 985, 643, 1217, 78, 16, 380, 223, 177, 600, 349, 12, 776, 26, 1738, 526, 85,
        1542, 111, 844, 93, 595, 1545, 873, 836, 422, 180, 1187, 329, 231, 1521, 54, 162, 212, 471,
        1329, 156, 1299, 160, 541, 676, 67, 200, 22, 24, 76, 242, 178, 1093, 1173, 818, 1380, 284,
        335, 642, 1047, 112, 271, 541, 927, 52, 983, 238, 116, 135, 871, 400, 436, 1094, 684, 249,
        263, 303, 24, 437, 813, 32, 45, 19, 620, 57, 866, 44, 68, 277, 1112, 110, 77, 1481, 437,
        302, 678, 541, 904, 322, 13, 186, 1474, 836, 43, 1020, 201, 1586, 1169, 1149, 470, 535, 55,
        879, 133, 1229, 106, 989, 1023, 256, 103, 56, 401, 667, 557, 98, 288, 694, 286, 237, 1661,
        933, 1063, 20, 227, 80, 815, 289, 1414, 234, 517, 227, 616, 829, 191, 1211, 92, 591, 279,
        22, 139, 67, 214, 60, 145, 468, 10, 521, 807, 1243, 76, 163, 190, 122, 804, 88, 383, 319,
        1127, 399, 376, 423, 304, 126, 10, 297, 377, 1103, 691, 139, 70, 519, 16, 15, 43, 397, 468,
        1183, 90, 28, 1262, 151, 1448, 62, 64, 1072, 386, 1330, 1313, 12, 100, 657, 28, 55, 612,
        337, 1865, 704, 263, 565, 249, 564, 565, 1218, 40, 1146, 150, 718, 1253, 228, 120, 713,
        925, 159, 36, 1087, 1023, 1490, 316, 540, 1124, 1127, 781, 417, 656, 0, 174, 1006, 529,
        389, 86, 90, 78, 403, 1500, 253, 35, 655, 650, 933, 815, 108, 168, 321, 345, 147, 251, 258,
        25, 173, 243, 740, 48, 476, 1507, 634, 425, 738, 160, 1415, 395, 448, 156, 636, 1967, 516,
        316, 628, 810, 817, 26, 20, 753, 22, 1133, 352, 204, 211, 47, 22, 874, 43, 12, 18, 1015,
        779, 108, 579, 251, 1398, 33, 1507, 93, 274, 904, 221, 1062, 868, 3, 363, 42, 14, 435, 62,
        1508, 540, 64, 267, 1690, 418, 205, 502, 152, 142, 414, 178, 50, 344, 780, 81, 635, 128,
        355, 239, 1708, 1814, 29, 251, 624, 22, 38, 789, 948, 186, 529, 895, 76, 150, 416, 502,
        975, 1216, 456, 862, 522, 1149, 131, 10, 121, 1353, 313, 568, 595, 6, 318, 633, 331, 1652,
        656, 214, 21, 35, 289, 80, 860, 229, 244, 1188, 350, 594, 424, 235, 327, 6, 1083, 40, 134,
        839, 279, 172, 1452, 197, 47, 2, 73, 607, 238, 1151, 844, 533, 110, 1207, 125, 129, 16,
        1000, 965, 236, 228, 497, 589, 111, 1245, 453, 179, 956, 116, 212, 47, 497, 380, 574, 355,
        799, 209, 384, 47, 449, 688, 312, 748, 1531, 1092, 23, 1001, 69, 155, 924, 1352, 163, 1561,
        743, 609, 1261, 1231, 32, 1, 739, 513, 300, 370, 36, 568, 89, 487, 201, 11, 146, 274, 163,
        1029, 829, 469, 299, 118, 732, 769, 120, 1093, 776, 610, 1944, 90, 67, 494, 831, 88, 227,
        1257, 344, 662, 401, 310, 664, 56, 94, 183, 935, 179, 643, 4, 1083, 567, 1525, 208, 204,
        899, 123, 36, 438, 1171, 265, 1406, 177, 202, 1398, 631, 444, 385, 589, 29, 124, 96, 237,
        374, 793, 794, 502, 665, 287, 575, 113, 305, 157, 465, 376, 66, 662, 77, 595, 75, 141, 243,
        254, 30, 5, 622, 140, 443, 566, 360, 192, 1531, 1113, 1299, 598, 147, 469, 732, 1565, 409,
        1380, 550, 173, 232, 361, 131, 99, 37, 547, 132, 1779, 193, 228, 664, 553, 568, 389, 1069,
        58, 71, 610, 738, 624, 261, 491, 158, 105, 416, 131, 198, 35, 823, 9, 313, 6, 429, 1492,
        290, 313, 272, 281, 427, 280, 661, 141, 54, 383, 3, 130, 43, 418, 2, 1040, 1051, 1006, 38,
        151, 1325, 1357, 117, 1473, 175, 201, 613, 1458, 1218, 588, 169, 228, 565, 901, 420, 42,
        117, 110, 442, 9, 99, 1685, 979, 84, 35, 129, 248, 1, 21, 360, 123, 203, 1320, 1200, 209,
        510, 362, 106, 148, 313, 292, 63, 842, 93, 88, 134, 720, 565, 156, 118, 983, 119, 1451,
        757, 736, 445, 466, 226, 265, 573, 612, 652, 170, 225, 32, 1049, 1332, 366, 1375, 692, 270,
        388, 321, 1153, 909, 1266, 93, 5, 495, 377, 212, 429, 90, 199, 278, 631, 693, 63, 816, 395,
        281, 315, 0, 737, 575, 121, 865, 1, 485, 262, 49, 804, 518, 109, 600, 358, 221, 14, 370,
        450, 947, 448, 67, 576, 22, 1266, 226, 100, 10, 607, 620, 295, 568, 316, 51, 687, 199,
        1478, 45, 489, 1878, 1035, 298, 219, 363, 85, 664, 1290, 492, 70, 644, 78, 163, 100, 102,
        465, 732, 439, 93, 25, 847, 297, 172, 361, 393, 304, 461, 583, 122, 121, 762, 58, 112, 85,
        142, 48, 193, 1617, 386, 685, 1054, 584, 488, 394, 665, 277, 263, 596, 290, 1231, 171,
        1394, 9, 1218, 77, 54, 487, 182, 528, 695, 662, 413, 345, 51, 690, 1702, 203, 1500, 461,
        1755, 190, 371, 1122, 1614, 324, 238, 569, 1482, 15, 711, 1332, 700, 437, 242, 174, 642,
        660, 987, 1232, 121, 620, 17, 389, 22, 105, 847, 36, 251, 285, 1238, 162, 1227, 1473, 411,
        66, 258, 377, 1135, 438, 117, 664, 281, 1070, 301, 132, 256, 498, 172, 194, 103, 662, 606,
        342, 340, 1501, 802, 549, 380, 58, 179, 361,
    ];

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let results: Vec<(u32, u32)> = (min..max)
        .map(|pos| (pos, simulate_move(&input, pos)))
        .collect();

    let result = results.iter().min_by_key(|(_, result)| result).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_calculate_fuel() {
    assert_eq!(calculate_fuel(16, 5), 66);
    assert_eq!(calculate_fuel(1, 5), 10);
    assert_eq!(calculate_fuel(2, 5), 6);
}
