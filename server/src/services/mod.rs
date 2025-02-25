/*
 * Created on Sun May 16 2021
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2021, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

pub mod bgsave;
pub mod snapshot;
use crate::corestore::memstore::Memstore;
use crate::diskstore::flock::FileLock;
use crate::storage;
use crate::util::os;
use crate::IoResult;

pub fn restore_data(src: Option<String>) -> IoResult<()> {
    if let Some(src) = src {
        // hmm, so restore it
        os::recursive_copy(src, "data")?;
        log::info!("Successfully restored data from snapshot");
    }
    Ok(())
}

pub fn pre_shutdown_cleanup(mut pid_file: FileLock, mr: Option<&Memstore>) -> bool {
    if let Err(e) = pid_file.unlock() {
        log::error!("Shutdown failure: Failed to unlock pid file: {}", e);
        return false;
    }
    if let Some(mr) = mr {
        log::info!("Compacting tree");
        if let Err(e) = storage::v1::interface::cleanup_tree(mr) {
            log::error!("Failed to compact tree: {}", e);
            return false;
        }
    }
    true
}
