/// this frame is supposed to be repeated twice when cycling on the animation
const IDLE1: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 16, 8, 8, 4, 4, 4, 8, 48,
    64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96, 48, 24,
    12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12, 24, 48,
    96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 100, 130, 2, 2, 2, 2, 2, 1,
    0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 192, 193, 193, 194, 4, 8,
    16, 32, 64, 128, 0, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16,
    8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0,
    248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148, 140,
    109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 56, 4, 3, 0,
    0, 0, 0, 0, 0, 0, 12, 12, 12, 13, 1, 0, 64, 160, 33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4, 4,
    8, 8, 16, 16, 16, 16, 16, 17, 15, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    170, 170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31,
    120, 192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152,
    219, 124, 56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2, 2, 1,
    1, 1, 1, 1, 1, 2, 2, 4, 4, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7,
    159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24, 16, 49,
    35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
];

const IDLE3: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 8, 4, 2, 2, 4, 24, 96,
    128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96, 48, 24,
    12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12, 24, 48,
    96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 194, 1, 1, 2, 2, 4, 4, 2, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 96, 0, 129, 130, 130, 132, 8, 16, 32, 64,
    128, 0, 0, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 8, 8,
    8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0, 248, 14,
    31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148, 140, 109, 31,
    14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112, 25, 6, 0, 0, 0,
    0, 0, 0, 0, 24, 24, 24, 27, 3, 0, 64, 160, 34, 36, 20, 18, 18, 18, 11, 8, 8, 8, 8, 5, 5, 9, 9,
    16, 16, 16, 16, 16, 17, 15, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170,
    170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120,
    192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152, 219,
    124, 56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2, 2, 1, 1,
    1, 1, 1, 1, 2, 2, 4, 4, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7,
    159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24, 16, 49,
    35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
];

const IDLE4: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 128, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 4, 2, 1, 1, 2, 12,
    48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96,
    48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12,
    24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 225, 0, 0, 1, 1, 2,
    2, 1, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 192, 193, 193, 194,
    4, 8, 16, 32, 64, 128, 0, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16, 16,
    16, 8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1,
    0, 248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148, 140,
    109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112, 12, 3,
    0, 0, 0, 0, 0, 0, 0, 12, 12, 12, 13, 1, 0, 64, 160, 33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4,
    4, 8, 8, 16, 16, 16, 16, 16, 17, 15, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 170, 170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0,
    31, 120, 192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20,
    152, 219, 124, 56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2,
    2, 1, 1, 1, 1, 1, 1, 2, 2, 4, 4, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135,
    31, 7, 159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24,
    16, 49, 35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
];

const IDLE5: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 8, 4, 2, 2, 2, 4, 56,
    64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96, 48, 24,
    12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12, 24, 48,
    96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 226, 1, 1, 2, 2, 2, 2, 1, 0,
    0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 192, 193, 193, 194, 4, 8, 16,
    32, 64, 128, 0, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 8,
    8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0, 248,
    14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148, 140, 109,
    31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112, 12, 3, 0, 0,
    0, 0, 0, 0, 0, 12, 12, 12, 13, 1, 0, 64, 160, 33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4, 4, 8,
    8, 16, 16, 16, 16, 16, 17, 15, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170,
    170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120,
    192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152, 219,
    124, 56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2, 2, 1, 1,
    1, 1, 1, 1, 2, 2, 4, 4, 8, 8, 8, 8, 8, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7,
    159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12, 24, 16, 49,
    35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
];

const PREP: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 128, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 4, 2, 1, 1, 2, 12,
    48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96,
    48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12,
    24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 225, 0, 0, 1, 1, 2,
    2, 129, 128, 128, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 0, 1, 225, 26,
    6, 9, 49, 53, 1, 138, 124, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16,
    16, 16, 8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15,
    1, 0, 248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148,
    140, 109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112,
    12, 3, 0, 0, 24, 6, 5, 152, 153, 132, 195, 124, 65, 65, 64, 64, 32, 33, 34, 18, 17, 17, 17, 9,
    8, 8, 8, 8, 4, 4, 4, 4, 4, 4, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 170, 170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0,
    0, 0, 31, 120, 192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20,
    20, 152, 219, 124, 56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3,
    2, 2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 130,
    135, 31, 7, 159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14, 12,
    24, 16, 49, 35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1, 15, 0, 15, 2, 5, 8,
];

const TAP1: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 128, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 4, 2, 1, 1, 2, 12,
    48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 248, 248, 248, 248, 0, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0,
    192, 96, 48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198,
    132, 12, 24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 225, 0, 0,
    1, 1, 2, 2, 129, 128, 128, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 0, 1,
    1, 2, 4, 8, 16, 32, 67, 135, 7, 1, 0, 184, 188, 190, 159, 95, 95, 79, 76, 32, 32, 32, 32, 16,
    16, 16, 16, 8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252,
    15, 1, 0, 248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148,
    148, 140, 109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128,
    112, 12, 3, 0, 0, 24, 6, 5, 152, 153, 132, 67, 124, 65, 65, 64, 64, 32, 33, 34, 18, 17, 17, 17,
    9, 8, 8, 8, 8, 4, 4, 8, 8, 16, 16, 16, 16, 16, 17, 15, 1, 61, 124, 252, 252, 252, 252, 252, 60,
    12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170, 255, 255, 195, 191, 127, 3, 127, 191, 195, 255,
    255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120, 192, 0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207,
    3, 255, 3, 207, 50, 18, 20, 20, 152, 219, 124, 56, 15, 0, 192, 120, 63, 16, 16, 16, 16, 8, 8,
    8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 2, 2, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 130, 135, 31, 7, 159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32,
    16, 16, 16, 17, 11, 14, 12, 24, 16, 49, 35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35,
    49, 16, 24, 12, 6, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2,
    1, 15, 0, 15, 2, 5, 8,
];

const TAP2: [u8; 636] = [
    0, 0, 126, 126, 24, 60, 102, 66, 0, 12, 28, 112, 112, 28, 12, 0, 116, 116, 20, 20, 124, 104, 0,
    124, 124, 0, 112, 120, 44, 36, 124, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 128, 0, 0, 0, 0, 0, 128, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16, 8, 4, 2, 1, 1, 2, 12,
    48, 64, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 0, 0, 0, 0, 192, 96,
    48, 24, 12, 132, 198, 98, 35, 51, 17, 145, 113, 241, 113, 145, 17, 51, 35, 98, 198, 132, 12,
    24, 48, 96, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 225, 0, 0, 1, 1, 2,
    2, 1, 0, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 48, 48, 0, 0, 1, 225, 26, 6, 9,
    49, 53, 1, 138, 124, 0, 0, 128, 128, 128, 128, 64, 64, 64, 64, 32, 32, 32, 32, 16, 16, 16, 16,
    8, 8, 8, 8, 8, 196, 4, 196, 4, 196, 2, 194, 2, 194, 1, 1, 1, 1, 0, 0, 0, 0, 0, 252, 15, 1, 0,
    248, 14, 31, 109, 140, 148, 148, 164, 166, 249, 224, 255, 224, 249, 166, 164, 148, 148, 140,
    109, 31, 14, 248, 0, 1, 15, 252, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 112, 12, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 64, 160, 33, 34, 18, 17, 17, 17, 9, 8, 8, 8, 8, 4, 4, 4,
    4, 4, 4, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 170, 170,
    255, 255, 195, 191, 127, 3, 127, 191, 195, 255, 255, 170, 170, 0, 0, 0, 0, 0, 0, 31, 120, 192,
    0, 15, 56, 124, 219, 152, 20, 20, 18, 50, 207, 3, 255, 3, 207, 50, 18, 20, 20, 152, 219, 124,
    56, 15, 0, 192, 120, 31, 16, 16, 16, 16, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 2, 3, 122, 122, 121,
    121, 121, 121, 57, 49, 2, 2, 4, 4, 8, 8, 8, 136, 136, 135, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    130, 135, 31, 7, 159, 7, 28, 7, 159, 7, 159, 7, 2, 130, 0, 0, 0, 0, 32, 16, 16, 16, 17, 11, 14,
    12, 24, 16, 49, 35, 98, 102, 68, 68, 71, 71, 71, 68, 68, 102, 98, 35, 49, 16, 24, 12, 6, 3, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 120, 124, 254, 255, 63,
    7, 0, 0, 0, 0, 255, 255, 127, 127, 63, 62, 28, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 8, 23, 0, 15, 1, 2, 1,
    15, 0, 15, 2, 5, 8,
];

use kyria_oled::*;

// after a lot of tests, with my compression function I found the most memory efficient way to
// store the frames is to use the IDLE4 frame as a base and then save the difference between the
// IDLE4 frame and the others
fn main() {
    let base = IDLE4;

    println!(
        "static char PROGMEM CURRENT_FRAME[{}] = {{ 0 }};",
        base.len()
    );

    print_slice_as_c_array("BASE_FRAME", &base);
    println!("#define IDLE4 BASE_FRAME");
    print_slice_as_c_array("BASE_TO_IDLE1", &compress(&diff(&base, &IDLE1)));
    println!(
        "// We want to repeat the next frame twice so we are going to make a fake IDLE2 frame"
    );
    println!("#define BASE_TO_IDLE2 BASE_TO_IDLE1");
    print_slice_as_c_array("BASE_TO_IDLE3", &compress(&diff(&base, &IDLE3)));
    println!("// BASE_TO_IDLE4 frame is almost empty since IDLE4 is already used as the base");
    print_slice_as_c_array("BASE_TO_IDLE4", &compress(&diff(&base, &IDLE4)));
    print_slice_as_c_array("BASE_TO_IDLE5", &compress(&diff(&base, &IDLE5)));

    print_slice_as_c_array("BASE_TO_PREP", &compress(&diff(&base, &PREP)));

    print_slice_as_c_array("BASE_TO_TAP1", &compress(&diff(&base, &TAP1)));
    print_slice_as_c_array("BASE_TO_TAP2", &compress(&diff(&base, &TAP2)));
}
