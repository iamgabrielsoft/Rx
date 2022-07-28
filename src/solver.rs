

use crate::dumpfile::{Operations, Operation};
use crate::error::*;
use crate::fileutils::{ PathList, is_same_file};
use crate::renamer::RenameMap;
use path_abs::PathAbs;
use std::path::PathBuf;


#[allow(dead_code)]

/** Renaming order function  */
pub fn solve_rename_order(rename_map: &RenameMap) -> Result<Operations> {
    //solve the renaming order of the files
    let mut level_list: Vec<usize> = rename_map
        .values()
        .map(|x| x.components()
        .count()).collect(); 
        
        level_list.sort_unstable(); 
        level_list.dedup(); 
        level_list.reverse(); 



    //sorting algorithm from highher to lower
    let mut rename_order = PathList::new(); 
    for level in level_list {
        let level_target:Vec<PathBuf> = rename_map.keys().filter_map(|p| {
            if p.components().count() == level {
                Some(p.clone())
            
            }else {
                None
            }
        }).collect(); 



        let mut existing_targets = get_existing_targets(&level_target, rename_map)?; 
        rename_order.append(
            &mut level_target.iter().filter_map(|p|
                if !existing_targets.contains(p) {
                    Some(p.clone())
                }else { None }
            ).collect(), 
        );


        
        //order and append to an entry
        match sort_existing_target(rename_map, &mut existing_targets) {
            Ok(mut targets) => rename_order.append(&mut targets), 
            Err(err) => return Err(err)
        }
    }

    let mut operations = Operations::new(); 

    
    for target in rename_order {
        operations.push(Operation {
            source: rename_map[&target].clone(), 
            target
        })
    }


    Ok(operations)
} 




pub fn revert_operations(operations: &[Operation]) -> Result<Operations> {
    let mut reverse_operations = operations.to_owned(); 
    reverse_operations.reverse();
    

    let inverse_operation = reverse_operations.into_iter().map(|Operation { source, target} | Operation {
        source, 
        target
    }).collect(); 

    Ok(inverse_operation)
}


/** get existing target on user system */
fn get_existing_targets(targets: &[PathBuf], rename_map: &RenameMap) -> Result<PathList> {
    let mut existing_target :PathList = Vec::new(); 

    for target in targets {
        if target.symlink_metadata().is_err() {
            continue;
        }
        

        if !rename_map.values().any(|x| x == target){
            let source = rename_map.get(target).cloned().unwrap(); 


            if is_same_file(&source, target) {
                continue; 
            }
            
            return Err(Error {
                kind:ErrorKind::ExistingPath,
                value: Some(format!("{} -> {}", source.display(), target.display()))
            })
        }

        existing_target.push(target.clone()); 
    }

    Ok(existing_target)
}



fn sort_existing_target(rename_map: &RenameMap, existing_target: &mut PathList) -> Result<PathList> {
    let mut ordered_target:PathList = Vec::new(); //a vector 

    while !existing_target.is_empty() {
        let mut selected_index: Option<usize>= None; //selected index

        let sources:PathList = existing_target.iter()
            .map(|x| rename_map.get(x).cloned().unwrap())
            .map(|p| PathAbs::new(p).unwrap().to_path_buf())
            .collect();


        //select without conflict 
        for (index, target) in existing_target.iter().enumerate() {
            let absolute_target = PathAbs::new(target).unwrap().to_path_buf(); 
            if !sources.contains(&absolute_target) {
                selected_index = Some(index); 
                break;
            } 
        }


        //store result in ordered targets 
        match selected_index {
            Some(index) => ordered_target.push(existing_target.swap_remove(index)),
            None =>  {
                return Err(Error {
                    kind: ErrorKind::SolveOrder, 
                    value: None 
                })
            },
        }
    }

    Ok(ordered_target)
}