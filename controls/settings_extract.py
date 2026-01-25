import xml.etree.ElementTree as ET

def main():
    tree = ET.parse("settings.xml")
    root = tree.getroot()

    for category_node in root[0]: # INPUT CATEGORIES MAPPINGS
        category = category_node.get("key")
        if not category: continue
        print(category)
        with open(f"../src/input/{category.lower()}.rs", 'w') as file:
            file.write(f"pub const CATEGORY: &'static str = &\"{category}\";\n")
            for input in category_node[0]:
                print("\t"+input.text)
                file.write(f"pub const {input.text.partition("_")[2] or input.text}: &'static str = &\"{input.text}\";\n")
        

if __name__ == "__main__":
    main()