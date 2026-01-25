#check default.xml
"""
category_prefixes = {
    "KEYBOARD": "IOMS_",
    "MOUSE_WHEEL": "IOMS_",
    "MOUSE_BUTTON": "IOMS_",
}
all of them needs to be prefixed with IOMS_
"""
#prefix is based on category
#only KEYBOARD keycodes had their prefix stripped from the fivem docs (this is aids)
keycode_prefixes = {
    "KEYBOARD": "KEY_",
    "DIGITALBUTTON_AXIS": "", #UNKNOWN
    "GAME_CONTROLLED": "", #UNKNOWN
    "JOYSTICK_AXIS": "", #already prefix from fivem docs
    "JOYSTICK_AXIS_NEGATIVE": "", #already prefix from fivem docs
    "JOYSTICK_AXIS_POSITIVE": "", #already prefix from fivem docs
    "JOYSTICK_BUTTON": "", #already prefix from fivem docs
    "JOYSTICK_IAXIS": "", #already prefix from fivem docs
    "JOYSTICK_POV": "", #already prefix from fivem docs
    "JOYSTICK_POV_AXIS": "", #already prefix from fivem docs
    "MKB_AXIS": "", #UNKNOWN
    "MOUSE_ABSOLUTEAXIS": "", #already prefix from fivem docs
    "MOUSE_BUTTON": "", #already prefix from fivem docs
    "MOUSE_BUTTONANY": "", #already prefix from fivem docs
    "MOUSE_CENTEREDAXIS": "", #already prefix from fivem docs
    "MOUSE_RELATIVEAXIS": "", #already prefix from fivem docs
    "MOUSE_SCALEDAXIS": "", #already prefix from fivem docs
    "MOUSE_NORMALIZED": "", #already prefix from fivem docs
    "MOUSE_WHEEL": "", #already prefix from fivem docs
    "PAD_ANALOGBUTTON": "", #already prefix from fivem docs
    "PAD_AXIS": "", #already prefix from fivem docs
    "PAD_DEBUGBUTTON": "", #already prefix from fivem docs
    "PAD_DIGITALBUTTON": "", #already prefix from fivem docs
    "PAD_DIGITALBUTTONANY": "", #already prefix from fivem docs
    "TOUCHPAD_ABSOLUTE_AXIS": "", #already prefix from fivem docs
    "TOUCHPAD_CENTERED_AXIS": "", #already prefix from fivem docs
}

the_numbers = [
    "ZERO",
    "ONE",
    "TWO",
    "THREE",
    "FOUR",
    "FIVE",
    "SIX",
    "SEVEN",
    "EIGHT",
    "NINE",
]
    
    
    

def prefix_keycode(category, keycode):
    return keycode_prefixes[category] + keycode

def number_check(stripped_keycode):
    if not stripped_keycode.isnumeric():
        return stripped_keycode

    num = int(stripped_keycode)
    if num > 9:
        return "ERROR OR SOMETHING: " + stripped_keycode

    return the_numbers[num]

def santize_desc(desc):
    return desc.replace("\\","\\\\").replace("\"", "\\\"")

def main():
    out = {}
    with open("keycodes.csv") as keycodes_file:
        current_category = ""
        for line in keycodes_file:
            parts = line.replace('\n','').split(',')
            category, keycode, desc = parts[0], parts[1], parts[2]

            if category:
                current_category = category
                print(f"category: {current_category} - {desc}")
                out[current_category] = []
                out[current_category].append(["CATEGORY",f"IOMS_{current_category}", f"{santize_desc(desc)}"])

            elif keycode:
                keycode = prefix_keycode(current_category, keycode)
                stripped = number_check(keycode.partition("_")[2] or keycode)
                print(f"keycode:  {stripped} - {keycode} - {desc}")
                out[current_category].append([stripped, keycode, santize_desc(desc)])

            else:
                print("error occured i think: {" + line + "}")


    print("\nWriting files...")
    for category in out.keys():
        if not category: continue
        with open(f"../src/keycode/{category.lower()}.rs", 'w') as file:
            #define consts
            for data in out[category]:
                stripped = data[0]
                code = data[1]
                desc = data[2]
                file.write(f"pub const {stripped}: &'static(&'static str, &'static str) = &(\"{code}\", \"{desc}\");\n")
            
            #"ALL" list
            file.write("pub const ALL: &'static[&'static(&'static str,&'static str)] = &[\n")
            for data in out[category]:
                stripped = data[0]
                code = data[1]
                desc = data[2]
                file.write(f"\t{stripped}, //{desc}\n")
            
            file.write("];\n")
    print("Done.")

if __name__ == "__main__":
    main()