import xml.etree.ElementTree as ET

def main():
    tree = ET.parse("settings.xml")
    root = tree.getroot()

    for category_node in root[0]: # INPUT CATEGORIES MAPPINGS
        category = category_node.get("key")
        if not category: continue
        print(category)
        with open(f"../src/input/{category.lower()}.rs", 'w') as file:
            file.write(f"pub const CATEGORY: &'static(&'static str, &'static str, &'static str) = &(\"{category}\", \"{' '.join(map(lambda elems : elems[0].upper() + elems[1:], category.lower().split('_')))}\", \"{' '.join(map(lambda elems : elems[0].upper() + elems[1:], category.lower().split('_')))}\");\n")
            for input in category_node[0]:
                stripped = input.text.partition("_")[2] or input.text
                pretty = ' '.join(map(lambda elems : elems[0].upper() + elems[1:], stripped.lower().split('_')))
                print("\t"+input.text)
                file.write(f"pub const {stripped}: &'static(&'static str, &'static str, &'static str) = &(\"{input.text}\", \"{pretty}\", \"{"Unknown"}\");\n")
                

            file.write("\n")

            file.write("pub const ALL: &'static[&'static(&'static str, &'static str, &'static str)] = &[\n")
            file.write(f"\tCATEGORY, // {category}\n")
            for input in category_node[0]:
                file.write(f"\t{input.text.partition("_")[2] or input.text},// {input.text}\n")
            file.write("];\n")
        

if __name__ == "__main__":
    main()