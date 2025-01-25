# LLM generated from XYText's decoding code (damn!)

import struct

def decrypt_u16(data, offset, c, k):
    c[0] = struct.unpack_from('<H', data, offset[0])[0] ^ k[0]
    offset[0] += 2
    k[0] = ((k[0] << 3) | (k[0] >> 13)) & 0xFFFF

def decrypt_var(data, offset, s, c, k):
    var_length = struct.unpack_from('<H', data, offset[0])[0] ^ k[0]
    offset[0] += 2
    k[0] = ((k[0] << 3) | (k[0] >> 13)) & 0xFFFF
    var_data = data[offset[0]:offset[0] + var_length * 2]
    offset[0] += var_length * 2
    var_str = var_data.decode('utf-16le')
    s[0] += f"\\v{var_str}"

def get_strings_from_file(path):
    with open(path, 'rb') as f:
        data = f.read()

    text_sections = struct.unpack_from('<H', data, 0)[0]
    line_count = struct.unpack_from('<H', data, 2)[0]
    if line_count == 0:
        return None

    total_length = struct.unpack_from('<I', data, 4)[0]
    initial_key = struct.unpack_from('<I', data, 8)[0]
    section_data = struct.unpack_from('<i', data, 12)[0]

    try:
        if initial_key != 0:
            raise ValueError("Invalid initial key! Not 0?")
        if section_data + total_length != len(data) or text_sections != 1:
            raise ValueError("Invalid Text File")

        section_length = struct.unpack_from('<I', data, section_data)[0]
        if section_length != total_length:
            raise ValueError("Section size and overall size do not match.")
    except ValueError:
        return None

    key = 0x7C89
    result = [None] * line_count

    for i in range(line_count):
        k = [key]
        s = [""]
        offset = [struct.unpack_from('<i', data, i * 8 + section_data + 4)[0] + section_data]
        length = struct.unpack_from('<H', data, i * 8 + section_data + 8)[0]
        start = offset[0]
        c = [0]

        while offset[0] < start + length * 2:
            decrypt_u16(data, offset, c, k)
            match c[0]:
                case 0:
                    break
                case 0x10:
                    decrypt_var(data, offset, s, c, k)
                case 0xE07F:
                    s[0] += chr(0x202F)  # nbsp
                case 0xE08D:
                    s[0] += chr(0x2026)  # …
                case 0xE08E:
                    s[0] += chr(0x2642)  # ♂
                case 0xE08F:
                    s[0] += chr(0x2640)  # ♀
                case _:
                    match c[0]:
                        case 0x2019:
                            s[0] += "'"
                        case 0x2014:
                            s[0] += "-"
                        case _:
                            s[0] += chr(c[0])
        result[i] = s[0]
      
        key = (key + 0x2983) & 0xFFFF
    return result

import sys
import json
print(json.dumps(get_strings_from_file(sys.argv[1]), indent=4))

# Example usage:
# strings = get_strings_from_file('path_to_file')
# if strings:
#     for s in strings:
#         print(s)