import os
import re

cur_path = os.getcwd()
BASEPATH = os.path.join(cur_path, "reliquary/src/network/gen/")

command_ids = []
with open(os.path.join(BASEPATH, "command_id.rs"), "r") as f:
    for line in f.readlines():
        c_match = re.search("(?<=^pub const )\w+(?=: u16)", line)
        if c_match is None:
            continue
        c = c_match[0]
        command_ids.append(c)
if command_ids is None or not command_ids:
    print("Error in reading command_id.rs")
    exit(-1)
print(command_ids)
commands = []

proto_folder_path = os.path.join(os.path.join(BASEPATH, "proto/"))
for c in [f for f in os.listdir(proto_folder_path) if os.path.isfile(os.path.join(proto_folder_path, f))]:
    if not c.endswith(".rs"):
        print("Skipped file", c)
        continue
    command_name = c.replace(".rs", "")
    if command_name in command_ids:
        commands.append(command_name)
    
header = ("""use std::any::Any;\n"""
          """use reliquary::network::GameCommand;\n"""
          """use reliquary::network::gen::command_id;\n\n""")
 
function_header = ("""pub fn command_id_to_struct(command: GameCommand) -> Option<protobuf::Result<dyn Any>> {\n"""
                   """    match command.command_id {\n""")
 
function_footer = ("""        _ => None\n"""
                   """    }\n"""
                   """}\n""")
 
uses = ""
function_inner = ""

for c in commands:
    uses += f"""use reliquary::network::gen::proto::{c}::{c};\n"""
    function_inner += (
        f"""        command_id::{c} => {{\n"""
        f"""            let cmd = command.parse_proto::<{c}>();\n"""
        """            Some(cmd)\n"""
        """        }\n"""
    )
 
with open(os.path.join(cur_path, "output.rs"), "w") as f:
    f.write(header)
    f.write(uses)
    f.write(function_header)
    f.write(function_inner)
    f.write(function_footer)

print("Output file created!")