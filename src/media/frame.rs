use opencv::{
    core::{
        Mat,
        ToOutputArray,
        ToInputArray,
        ToInputOutputArray,
        _OutputArray,
        _InputArray,
        _InputOutputArray,
        Point3_,
        CV_8UC3,
    },
    prelude::{
        MatTraitConst,
        MatTrait,
    }
};

use std::ops::{ Index, IndexMut };

use crate::logging::LogResult;

pub struct Frame(Mat);

#[derive(Clone, Copy)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> LogResult<Self> {
        Ok(Self(unsafe {
            Mat::new_rows_cols(height as i32, width as i32, CV_8UC3)?
        }))
    }

    pub fn width(&self) -> u32 {
        self.0.cols() as u32
    }

    pub fn height(&self) -> u32 {
        self.0.rows() as u32
    }
}

impl Index<(u32, u32)> for Frame {
    type Output = Color;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let pt = self.0
            .at_2d::<Point3_<u8>>(index.1 as i32, index.0 as i32)
            .unwrap();

        unsafe { std::mem::transmute(pt) }
    }
}

impl IndexMut<(u32, u32)> for Frame {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Color {
        let pt = self.0
            .at_2d_mut::<Point3_<u8>>(index.1 as i32, index.0 as i32)
            .unwrap();

        unsafe { std::mem::transmute(pt) }
    }
}

impl ToOutputArray for Frame {
    fn output_array(&mut self) -> opencv::Result<_OutputArray> {
            _OutputArray::from_mat_mut(&mut self.0)
    }
}

impl ToInputArray for Frame {
    fn input_array(&self) -> opencv::Result<_InputArray> {
            _InputArray::from_mat(&self.0)
    }
}

impl ToInputOutputArray for Frame {
    fn input_output_array(&mut self) -> opencv::Result<_InputOutputArray> {
            _InputOutputArray::from_mat_mut(&mut self.0)
    }
}