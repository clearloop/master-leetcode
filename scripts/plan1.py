import json;

def plan(sps, stags):
    problems = json.loads(sps)
    tags = json.loads(stags)
    ret = []

    tm = {};
    for tag in tags:
        tm[tag["tag"]] = tag["refs"];

    for i in problems:
        if i["level"] == 1 and str(i["id"]) in tm["linked-list"]:
            ret.append(str(i["id"]))

            
    return ret
