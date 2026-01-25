import json

def parse_mapping(xml):
    input_code = ""
    source_key_code = ""

    line = xml.readline().strip()
    while line != "</Item>":
        if line.startswith("<Input>"):
            input_code = line.replace("<Input>", "").replace("</Input>", "")

        if line.startswith("<Source>"):
            source_key_code = line.replace("<Source>", "").replace("</Source>", "") + ":"
        
        if line.startswith("<Item>"):
            source_key_code = source_key_code + line.replace("<Item>", "").replace("</Item>", "")
        

        line = xml.readline().strip()
    return (input_code, source_key_code)

def main():
    out = {}
    with open("default.xml") as xml:
        line = xml.readline().strip()
        line = xml.readline().strip()
        line = xml.readline().strip()
        while line:
            line = xml.readline().strip()
            if line != "<Item>":
                continue

            mapping = parse_mapping(xml)
            out[mapping[0]] = mapping[1]
    
    with open("default_binds.txt", "w") as out_file:
        json.dump(out, out_file, indent=2)
            




if __name__ == "__main__":
    main()