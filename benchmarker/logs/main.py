import json
import pprint

import sys


def main(path):
    # path = "raw-data-20240727_164500.json"
    rlist = []
    with open(path, mode="r",encoding="utf-8")as f:
        a = json.load(f)
        # pprint.pprint(a["metrics"])
        # pprint.pprint(a["metrics"])
        # print(len(a["metrics"]))

        for i in a["metrics"]:
            if a["metrics"][i]["type"] == "trend":
                # print(i.center(50,"="))
                # pprint.pprint(a["metrics"][i])
                rlist.append((a["metrics"][i]["values"]["med"],i,a["metrics"][i]))
    
    pprint.pprint(sorted(rlist,key=lambda a:a[0]))
    # pprint.pprint(rlist)

if __name__== "__main__":
    try:
        path = sys.argv[1]
    except :
        print("パスを入力してください")
    
    main(path)
