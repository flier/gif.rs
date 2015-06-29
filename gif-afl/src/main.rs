#![feature(plugin)]
#![plugin(afl_coverage_plugin)]

extern crate afl_coverage;

extern crate gif;
extern crate nom;

use nom::*;
use gif::*;
use gif::parser::*;
use gif::lzw::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use std::vec::Vec;

use std::fs::File;
use std::io::{self, Read};

fn main() {
  //println!("starting...");
  let decoded = decode_gif();
  //println!("done: {:?}", decoded);
}

//pub fn decode_gif () -> Option<Vec< Vec<(u8,u8,u8)> >> {
pub fn decode_gif () -> Option<usize> {
  let mut contents: Vec<u8> = Vec::new();
  let result = io::stdin().read_to_end(&mut contents).unwrap();
  let d = &contents[..];
  //let data = &d[13..];
  //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

  if let IResult::Done(data, logical_descriptor) = header_and_logical_screen_descriptor(d) {
    // we know the color table size
    match color_table(data, 256) {
      IResult::Done(_, colors) => {
        //println!("parsed: {:?}", colors);
        // allocate the image
        let mut buffer: Vec<u8> = Vec::with_capacity(400 * 300 * 3);
        unsafe { buffer.set_len(400 * 300 * 3); }

        let data = &d[801..];
        //println!("bytes:\n{}", &data[0..100].to_hex_from(8, d.offset(data)));

        match graphic_block(data) {
          IResult::Done(_, Block::GraphicBlock(opt_control, rendering)) => {
            //let (opt_control, rendering) = grb;
            match rendering {
              GraphicRenderingBlock::TableBasedImage(descriptor, code_size, blocks) => {
                match lzw::decode_lzw(colors, code_size as usize, blocks, &mut buffer[..]) {
                  Ok(nb) => {
                    //println!("decoded the image({} bytes):\n", nb);//, buffer.to_hex(8));
                    //return Some(buf_to_colors(&mut buffer[..], 400));
                    return Some(nb);
                    //panic!("correctly decoded")
                  },
                  _ => {
                    //panic!("could not decode")
                    return None;
                  }
                }
              },
              _ => {
                //panic!("plaintext extension");
                return None;
              }
            }
          },
          e  => {
            println!("error or incomplete: {:?}", e);
            //panic!("cannot parse graphic block");
            return None;
          }

        }
      },
      e  => {
        println!("error or incomplete: {:?}", e);
        //panic!("cannot parse global color table");
        return None;
      }
    }
  } else { None }
}
