use sdl2::pixels::{PixelFormat, PixelFormatEnum};
use serde::{Deserialize, Serialize};

use crate::PixelGrid;
use std::{
    cell::UnsafeCell, ops::{Add, Mul}, sync::atomic::{self, AtomicU32, Ordering::*}
};

thread_local! {
    static PIXEL_FORMAT: UnsafeCell<Option<PixelFormat>> = const { UnsafeCell::new(None) };
}

pub fn init_pixel_format_on_current_thread(pixel_format_enum: PixelFormatEnum) {
        PIXEL_FORMAT.with(|pixel_format| {
            unsafe { *pixel_format.get() = Some(PixelFormat::try_from(pixel_format_enum).unwrap()) }
        });
}

pub type Color = sdl2::pixels::Color;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Point2D(pub f64, pub f64); // A point in 2d space, represented as (x, y), with (0, 0) as the upper-left corner.

pub struct AtomicColor(AtomicU32);

impl Default for AtomicColor {
    fn default() -> Self {
        PIXEL_FORMAT.with(|pixel_format| {
            Self(AtomicU32::new(
                Color::BLACK.to_u32(unsafe { &pixel_format.get().as_ref().unwrap_unchecked().as_ref().unwrap_unchecked() })
            ))
        })
    }
}

impl AtomicColor {
    pub fn load(&self, order: atomic::Ordering) -> Color {
        PIXEL_FORMAT.with(|pixel_format| {
            Color::from_u32(unsafe { &pixel_format.get().as_ref().unwrap_unchecked().as_ref().unwrap_unchecked() }, self.0.load(order))
        })
    }

    pub fn load_u32(&self, order: atomic::Ordering) -> u32 {
        self.0.load(order)
    }

    pub fn store(&self, value: Color, order: atomic::Ordering) {
        self.0.store(
            PIXEL_FORMAT.with(|pixel_format| {
                value.to_u32(unsafe { &pixel_format.get().as_ref().unwrap_unchecked().as_ref().unwrap_unchecked() })
            }),
            order,
        );


    }
}


impl Mul<f64> for Point2D {
    type Output = Point2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Point2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Self) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point2D {
    pub fn draw_specifying_color(&self, target: &PixelGrid, color: Color) {
        target.0[self.1 as usize][self.0 as usize].store(color, Release);
        // The order of the coordinates is weird and unintuitive!
    }
}
