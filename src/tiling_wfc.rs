//! A WFC algorithm for tiling problems.

use crate::direction::*;
use crate::tile::*;
use crate::utils::vec2d::*;
use crate::wfc::*;
use crate::Real;

/// Options passed to the tiling WFC.
pub struct TilingWFCOptions {
    is_periodic: bool,
}

/// The data needed for the WFc algorithm
pub struct TilingWFC<T> {
    /// The problem tiles
    tiles: Vec<Tile<T>>,
    /// Associate an oriented tile id to its tile number, and its orientation
    id_to_oriented_tiles: Vec<(usize, usize)>,
    /// The underlying WFC problem
    wfc: WFC,
}

impl<T: Copy> TilingWFC<T> {
    /// Create a new structure for a tiling WFC problem
    pub fn new(
        tiles: Vec<Tile<T>>,
        neighbors: &[[usize; 4]],
        height: usize,
        width: usize,
        options: TilingWFCOptions,
        seed: [u8; 16],
    ) -> TilingWFC<T> {
        let (id_to_oriented_tiles, oriented_tile_ids) = generate_oriented_tile_ids(&tiles);
        let propagator =
            generate_propagator(neighbors, &tiles, &id_to_oriented_tiles, &oriented_tile_ids);
        let wfc = WFC::new(
            options.is_periodic,
            seed,
            get_tiles_weights(&tiles),
            propagator,
            height,
            width,
        );

        TilingWFC {
            tiles,
            id_to_oriented_tiles,
            wfc,
        }
    }

    /// Translate the generic WFC result into the concatenation of the tiles
    fn id_to_tiling(&self, ids: Vec2D<usize>) -> Vec2D<T> {
        let size = self.tiles[0].data()[0].height();
        let mut tiling = Vec2D::new(
            size * ids.height(),
            size * ids.width(),
            &self.tiles[0].data()[0][0][0],
        );

        for i in 0..ids.height() {
            for j in 0..ids.width() {
                let (tile_id, orientation) = self.id_to_oriented_tiles[ids[i][j]];
                for y in 0..size {
                    for x in 0..size {
                        tiling[i * size + y][j * size + x] =
                            self.tiles[tile_id].data()[orientation][y][x]
                    }
                }
            }
        }

        tiling
    }

    /// Run the wfc algorithm
    pub fn run(&mut self) -> Option<Vec2D<T>> {
        self.wfc.run().map(|x| self.id_to_tiling(x))
    }

    /// Reset the WFC algorithm
    pub fn restart(&mut self, seed: [u8; 16]) {
        self.wfc.restart(seed);
    }
}

/// Generate mapping from id to oriented tiles and vice versa.
fn generate_oriented_tile_ids<T>(tiles: &[Tile<T>]) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let id_to_oriented_tile = tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| (0..tile.data().len()).map(move |j| (i, j)))
        .flatten()
        .collect();

    let mut id = 0;
    let oriented_tile_ids = tiles
        .iter()
        .map(|tile| {
            let v = (id..id + tile.data().len()).collect();
            id += tile.data().len();
            v
        })
        .collect();

    (id_to_oriented_tile, oriented_tile_ids)
}

/// Generate a propagator given the neighbors list
fn generate_propagator<T>(
    neighbors: &[[usize; 4]],
    tiles: &[Tile<T>],
    id_to_oriented_tile: &[(usize, usize)],
    oriented_tile_ids: &[Vec<usize>],
) -> Vec<DirArray<Vec<usize>>> {
    let nb_oriented_tiles = id_to_oriented_tile.len();
    let mut dense_propagator =
        vec![DirArray::new(&vec![false; nb_oriented_tiles]); nb_oriented_tiles];

    for neighbor in neighbors {
        let tile1 = neighbor[0];
        let orientation1 = neighbor[1];
        let tile2 = neighbor[2];
        let orientation2 = neighbor[3];

        let action_map1 = generate_action_map(tiles[tile1].symmetry());
        let action_map2 = generate_action_map(tiles[tile2].symmetry());

        let mut add = |action: usize, direction| {
            let temp_orientation1 = action_map1[action][orientation1];
            let temp_orientation2 = action_map2[action][orientation2];
            let oriented_tile_id1 = oriented_tile_ids[tile1][temp_orientation1];
            let oriented_tile_id2 = oriented_tile_ids[tile2][temp_orientation2];
            dense_propagator[oriented_tile_id1][direction][oriented_tile_id2] = true;
            dense_propagator[oriented_tile_id2][direction.opposite()][oriented_tile_id2] = true;
        };

        add(0, Direction::Right);
        add(1, Direction::Down);
        add(2, Direction::Left);
        add(3, Direction::Up);
        add(4, Direction::Left);
        add(5, Direction::Up);
        add(6, Direction::Right);
        add(7, Direction::Down);
    }

    // Transform the dense propagator into a sparse one
    dense_propagator
        .into_iter()
        .map(|v_d| {
            v_d.map(|v| {
                v.into_iter()
                    .enumerate()
                    .filter(|(_, b)| *b)
                    .map(|(v, _)| v)
                    .collect()
            })
        })
        .collect()
}

/// Get the weight of all oriented tiles
fn get_tiles_weights<T>(tiles: &[Tile<T>]) -> Vec<Real> {
    tiles
        .iter()
        .map(|tile| {
            std::iter::repeat(tile.weight() / (tile.data().len() as f32)).take(tile.data().len())
        })
        .flatten()
        .collect()
}
