use cgmath::Vector3;

use mc_render::model::block::RenderableBlock;
use mc_render::model::block::World;
use mc_render::model::biome::BiomeColor;


use crate::loader::*;


pub struct TileWorld<'a> {
    tile: Tile,
    water_models: Vec<Model>,
    air_props: BlockProps,
    tile_view: TileView<'a>,
    biome_color_gen: &'a BiomeColor,
}

impl<'a> TileWorld<'a> {

    fn gen(&'a self, block: LayerView<'a>, element: ElementView<'a>) -> TileBlock<'a> {
        let (model, props) = self.tile.get_model(block.blockstate_id());
        TileBlock {
            model: model.as_slice(),
            water: self.water_models.as_slice(),
            props,
            light: block.light(),
            color: props.biome_color.get_inner_color(self.biome_color_gen, element.biome(), block.height()),
        }
    }

    fn air(&'a self) -> TileBlock<'a> {
        TileBlock {
            model: &self.water_models[0..0],
            water: &self.water_models[..],
            props: &self.air_props,
            light: 0,
            color: [0, 0, 0]
        }
    }
}

impl<'a> World for TileWorld<'a> {
    type Block = TileBlock<'a>;

    fn get(&self, loc: &Vector3<i32>) -> Self::Block {
        if loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x > 255 || loc.y > 255 || loc.z > 255 {
            let element = self.tile_view.element(loc.x, loc.z);
            let block = element.ceil();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.seafloor();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.shading();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.vegetation();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
        }
        self.air()
    }
    
    fn is_air(&self, loc: &Vector3<i32>) -> bool {
        if loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x > 255 || loc.y > 255 || loc.z > 255 {
            let element = self.tile_view.element(loc.x, loc.z);
            let block = element.ceil();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.seafloor();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.shading();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.vegetation();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
        }
        true
    }

}


pub struct TileBlock<'a> {
    model: &'a [Model],
    water: &'a [Model],
    props: &'a BlockProps,
    light: u8,
    color: [u8; 3]
}


impl<'b> RenderableBlock for TileBlock<'b> {
    type Model = Model;

    fn is_air(&self) -> bool {
        self.props.air
    }

    fn is_water(&self) -> bool {
        self.props.water
    }

    fn is_water_logged(&self) -> bool {
        self.props.waterlogged
    }

    fn get_models<'a>(&'a self) -> std::slice::Iter<'a, Self::Model> {
        self.model.iter()
    }

    fn get_water_models<'a>(&'a self) -> std::slice::Iter<'a, Self::Model> {
        self.water.iter()
    }

    fn get_inline_color(&self, tintindex: usize) -> [u8; 4] {
        let _ = tintindex;
        [self.color[0], self.color[1], self.color[2], 255]
    }

    fn get_light(&self) -> u8 {
        self.light
    }
}