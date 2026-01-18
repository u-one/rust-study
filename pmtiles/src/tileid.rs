#![allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileId {
    value: u64,
}

impl TileId {
    pub fn new(value: u64) -> TileId {
        TileId { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn encode(z: u8, x:u32, y:u32) -> TileId {
        // zoom levelまでのセル数の合計
        let mut id = (4u64.pow(z as u32) -1) /3;

        id += Self::xy_to_hilbert(z, x, y);
        TileId { value: id }
    }

    fn xy_to_hilbert(z: u8, x: u32, y: u32) -> u64 {
        if z == 0 {
            return 0;
        }
        let mut d = 0u64;
        let mut n = 1u32 << z;
        let mut s = n / 2;
        let mut x = x;
        let mut y = y;

        while s > 0 {
            let rx = ((x & s) > 0) as u32;
            let ry = ((y & s) > 0) as u32;
            d += (s as u64) * (s as u64) * ((3 * rx) ^ ry) as u64;
            Self::rotate(n, &mut x, &mut y, rx, ry);
            s = s >> 1;
        }
        d
    }

    /// Decodes a 64-bit TileID into its zoom level, x, and y components.
    pub fn decode(&self) -> (u8, u32, u32) {
        let mut id: u64 = self.value;
        let mut z = 0;
        loop {
            let next_z_offset = 4u64.pow(z as u32);
            if id < next_z_offset {
                break;
            }
            id -= next_z_offset;
            z += 1;
        }
        let (x, y) = Self::hilbert_to_xy(z, id);
        (z, x, y)
    } 

    // ヒルベルト曲線
    fn hilbert_to_xy(z: u8, mut d: u64) -> (u32, u32) {
        /* 1. 処理の全体像
         *   1. 象限の特定： 
         *   　１次元のインデックスdのうち、現在のステップで注目する2bitを見て、4つのエリアのどれに属するかを特定
         *   2. 座標の確定：
         *   　そのエリアに対応するx, yのオフセットを加える
         *   3. 座標系の回転・反転：
         *   　ヒルベルト曲線は一筆書きを維持するために、特定の象限に入ったとき、次ステップ以降の座標軸を回転・反転させる。
         *   　これによって隣り合うセルが常に連続するようになる
         * 
         * 2. ヒルベルト曲線の「4つの象限」
         *   1つの正方形を4分割したとき、ヒルベルト曲線は以下の順（U字型）に巡る。
         *   - 左下 (Lower-Left)
         *   - 左上 (Upper-Left)
         *   - 右上 (Upper-Right)
         *   - 右下 (Lower-Right)
         *   これをビット（d の下位2ビット）で表現すると、通常は 00, 01, 10, 11 の順に対応
         */
        let mut x = 0;
        let mut y = 0;
        let mut s = 1;
        let n = 1 << z;
        while s < n {
            /*
             * +-----------------+-----------------+
             * | 1               | 2               |
             * | (01)            | (10)            |
             * | (rx,ry) = (0,1) | (rx,ry) = (1,1) |
             * +-----------------+-----------------+
             * | 0               | 3               |
             * | (00)            | (11)            |
             * | (rx,ry) = (0,0) | (rx,ry) = (1,0) |
             * +-----------------+-----------------+
             * つまり↓
             * |d(10進数)|d(2進数)|rx|ry|場所|
             * +--------+--------+--+--+----+
             * |0       |00      |0 |0 |左下|
             * |1       |01      |0 |1 |左上|
             * |2       |10      |1 |1 |右上|
             * |3       |11      |1 |0 |右下|
             * +--------+--------+--+--+----+
             */
            // dから現在の次数における相対的なx, y(0 or 1)を抽出
            let rx = 1 & (d /2) as u32; // dの左側のビット
            let ry = 1 & (d as u32 ^ rx); // dの右側のビットとrxのXOR
            Self::rotate(s, &mut x, &mut y, rx, ry);
            x += s * rx;
            y += s * ry;
            d = d >> 2; // 2bit右シフト
            s *= 2;
        }
        (x, y)
    }

    fn rotate(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
        /*
         * 左下は、上に向かう線を右に向ける
         * 右下は、下に向かう線を左に向ける
         * 
         * 回転前
         * +---+   +---+
         * |   |   |   |
         *
         * +---+   +---+
         * |   |   |   |
         *
         * 回転後
         * +---+   +---+
         * |   |   |   |
         *
         * +---+   +---+
         *     |   |    
         * +---+   +---+
         *
         */
        if ry == 0 {
            if rx == 1 {
                // 右下に入った場合、上下左右を反転
                *x = n - 1 - *x;
                *y = n - 1 - *y;
            }
            // 左下または右下に入った場合、xとyを入れ替える（転置）
            std::mem::swap(x, y);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_tileid() {
        assert_eq!(TileId::new(0).decode(), (0, 0, 0));
        assert_eq!(TileId::new(1).decode(), (1, 0, 0));
        assert_eq!(TileId::new(2).decode(), (1, 0, 1));
        assert_eq!(TileId::new(3).decode(), (1, 1, 1));
        assert_eq!(TileId::new(4).decode(), (1, 1, 0));
        assert_eq!(TileId::new(5).decode(), (2, 0, 0));
        assert_eq!(TileId::new(19078479).decode(), (12, 3423, 1763));
    }

    #[test]
    fn encode_tileid() {
        assert_eq!(TileId::encode(0, 0, 0).value(), 0);
        assert_eq!(TileId::encode(1, 0, 0).value(), 1);
        assert_eq!(TileId::encode(1, 0, 1).value(), 2);
        assert_eq!(TileId::encode(1, 1, 1).value(), 3);
        assert_eq!(TileId::encode(1, 1, 0).value(), 4);
        assert_eq!(TileId::encode(2, 0, 0).value(), 5);
        assert_eq!(TileId::encode(12, 3423, 1763).value(), 19078479);
        //assert_eq!(TileId::encode(16, 55234, 27904).value(), 1);

    }
}