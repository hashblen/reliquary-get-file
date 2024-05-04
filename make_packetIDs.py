import re
import json

dict = {}
with open("CmdID.java", "r") as f:
    while f.readline().strip() != "// Cmd Ids":
        pass
    for line in f.readlines():
        line = line.strip()
        name_match = re.search("(?<=public static final int )\w+(?= )", line)
        id_match = re.search("(?<== )[0-9]+(?=;)", line)
        if name_match is None or id_match is None:
            continue
        dict[id_match[0]] = name_match[0]
with open("packetIDs.json", "w") as f:
    json.dump(dict, f)