// THIS IS UNUSED CODE LEFT FOR REFERENCE

// pub fn init_database() -> Connection {
//   let connection = Connection::open("assets/voxels.db").unwrap();

//   connection.execute(
//       "CREATE TABLE IF NOT EXISTS voxel_type (
//                 id TEXT PRIMARY KEY,
//                 is_opaque INTEGER,
//                 is_solid INTEGER
//                 );",
//       [],
//   ).unwrap();

//   connection
// }

// fn load_voxel_properties(connection: &Connection) -> HashMap<String, VoxelProperties> {
//   let mut statement = connection.prepare("SELECT id, is_opaque, is_solid FROM voxel_type").unwrap();
//   let voxels = statement.query_map(params![], |row| {
//       Ok(VoxelProperties {
//           id: row.get(0)?,
//           is_opaque: row.get::<_, i32>(1)? != 0,
//           is_solid: row.get::<_, i32>(2)? != 0,
//       })
//   }).unwrap();

//   voxels
//     .filter_map(Result::ok)
//     .map(|properties| (properties.id.clone(), properties))
//     .collect()
// }