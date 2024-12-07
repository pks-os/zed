use collections::HashMap;

// On some keyboards (e.g. German QWERTZ) it is not possible to type the full ASCII range
// without using option. This means that some of our built in keyboard shortcuts do not work
// for those users.
//
// The way macOS solves this problem is to move shortcuts around so that they are all reachable,
// even if the mnemoic changes. https://developer.apple.com/documentation/swiftui/keyboardshortcut/localization-swift.struct
//
// For example, cmd-> is the "switch window" shortcut because the > key is right above tab.
// To ensure this doesn't cause problems for shortcuts defined for a QWERTY layout, apple moves
// any shortcuts defined as cmd-> to cmd-:. Coincidentally this s also the same keyboard position
// as cmd-> on a QWERTY layout.
//
// Another example is cmd-[ and cmd-], as they cannot be typed without option, those keys are remapped to cmd-ö
// and cmd-ä. These shortcuts are not in the same position as a QWERTY keyboard, because on a QWERTZ keyboard
// the + key is in the way; and shortcuts bound to cmd-+ are still typed as cmd-+ on either keyboard (though the
// specific key moves)
//
// As far as I can tell, there's no way to query the mappings Apple uses except by rendering a menu with every
// possible key combination, and inspecting the UI to see what it rendered. So that's what we did...
//
// These mappings were generated by running https://github.com/ConradIrwin/keyboard-inspector, tidying up the
// output to remove languages with no mappings and other oddities, and converting it to a less verbose representation with:
//  jq -s 'map(to_entries | map({key: .key, value: [(.value | to_entries | map(.key) | join("")), (.value | to_entries | map(.value) | join(""))]}) | from_entries) | add'
// From there I used multi-cursor to produce this match statement.
#[cfg(target_os = "macos")]
pub fn get_key_equivalents(layout: &str) -> Option<HashMap<char, char>> {
    let mappings: &[(char, char)] = match layout {
        "com.apple.keylayout.ABC-AZERTY" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('.', ';'),
            ('/', ':'),
            ('0', 'à'),
            ('1', '&'),
            ('2', 'é'),
            ('3', '"'),
            ('4', '\''),
            ('5', '('),
            ('6', '§'),
            ('7', 'è'),
            ('8', '!'),
            ('9', 'ç'),
            (':', '°'),
            (';', ')'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', '^'),
            ('\'', 'ù'),
            ('\\', '`'),
            (']', '$'),
            ('^', '6'),
            ('`', '<'),
            ('{', '¨'),
            ('|', '£'),
            ('}', '*'),
            ('~', '>'),
        ],
        "com.apple.keylayout.ABC-QWERTZ" => &[
            ('"', '`'),
            ('#', '§'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', 'ß'),
            (':', 'Ü'),
            (';', 'ü'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '´'),
            ('\\', '#'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '\''),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Albanian" => &[
            ('"', '\''),
            (':', 'Ç'),
            (';', 'ç'),
            ('<', ';'),
            ('>', ':'),
            ('@', '"'),
            ('\'', '@'),
            ('\\', 'ë'),
            ('`', '<'),
            ('|', 'Ë'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Austrian" => &[
            ('"', '`'),
            ('#', '§'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', 'ß'),
            (':', 'Ü'),
            (';', 'ü'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '´'),
            ('\\', '#'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '\''),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Azeri" => &[
            ('"', 'Ə'),
            (',', 'ç'),
            ('.', 'ş'),
            ('/', '.'),
            (':', 'I'),
            (';', 'ı'),
            ('<', 'Ç'),
            ('>', 'Ş'),
            ('?', ','),
            ('W', 'Ü'),
            ('[', 'ö'),
            ('\'', 'ə'),
            (']', 'ğ'),
            ('w', 'ü'),
            ('{', 'Ö'),
            ('|', '/'),
            ('}', 'Ğ'),
        ],
        "com.apple.keylayout.Belgian" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('.', ';'),
            ('/', ':'),
            ('0', 'à'),
            ('1', '&'),
            ('2', 'é'),
            ('3', '"'),
            ('4', '\''),
            ('5', '('),
            ('6', '§'),
            ('7', 'è'),
            ('8', '!'),
            ('9', 'ç'),
            (':', '°'),
            (';', ')'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', '^'),
            ('\'', 'ù'),
            ('\\', '`'),
            (']', '$'),
            ('^', '6'),
            ('`', '<'),
            ('{', '¨'),
            ('|', '£'),
            ('}', '*'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Brazilian-ABNT2" => &[
            ('"', '`'),
            ('/', 'ç'),
            ('?', 'Ç'),
            ('\'', '´'),
            ('\\', '~'),
            ('^', '¨'),
            ('`', '\''),
            ('|', '^'),
            ('~', '"'),
        ],
        "com.apple.keylayout.Brazilian-Pro" => &[('^', 'ˆ'), ('~', '˜')],
        "com.apple.keylayout.British" => &[('#', '£')],
        "com.apple.keylayout.Canadian-CSA" => &[
            ('"', 'È'),
            ('/', 'é'),
            ('<', '\''),
            ('>', '"'),
            ('?', 'É'),
            ('[', '^'),
            ('\'', 'è'),
            ('\\', 'à'),
            (']', 'ç'),
            ('`', 'ù'),
            ('{', '¨'),
            ('|', 'À'),
            ('}', 'Ç'),
            ('~', 'Ù'),
        ],
        "com.apple.keylayout.Croatian" => &[
            ('"', 'Ć'),
            ('&', '\''),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            (':', 'Č'),
            (';', 'č'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'š'),
            ('\'', 'ć'),
            ('\\', 'ž'),
            (']', 'đ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Š'),
            ('|', 'Ž'),
            ('}', 'Đ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Croatian-PC" => &[
            ('"', 'Ć'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '\''),
            (':', 'Č'),
            (';', 'č'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'š'),
            ('\'', 'ć'),
            ('\\', 'ž'),
            (']', 'đ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Š'),
            ('|', 'Ž'),
            ('}', 'Đ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Czech" => &[
            ('!', '1'),
            ('"', '!'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('+', '%'),
            ('/', '\''),
            ('0', 'é'),
            ('1', '+'),
            ('2', 'ě'),
            ('3', 'š'),
            ('4', 'č'),
            ('5', 'ř'),
            ('6', 'ž'),
            ('7', 'ý'),
            ('8', 'á'),
            ('9', 'í'),
            (':', '"'),
            (';', 'ů'),
            ('<', '?'),
            ('>', ':'),
            ('?', 'ˇ'),
            ('@', '2'),
            ('[', 'ú'),
            ('\'', '§'),
            (']', ')'),
            ('^', '6'),
            ('`', '¨'),
            ('{', 'Ú'),
            ('}', '('),
            ('~', '`'),
        ],
        "com.apple.keylayout.Czech-QWERTY" => &[
            ('!', '1'),
            ('"', '!'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('+', '%'),
            ('/', '\''),
            ('0', 'é'),
            ('1', '+'),
            ('2', 'ě'),
            ('3', 'š'),
            ('4', 'č'),
            ('5', 'ř'),
            ('6', 'ž'),
            ('7', 'ý'),
            ('8', 'á'),
            ('9', 'í'),
            (':', '"'),
            (';', 'ů'),
            ('<', '?'),
            ('>', ':'),
            ('?', 'ˇ'),
            ('@', '2'),
            ('[', 'ú'),
            ('\'', '§'),
            (']', ')'),
            ('^', '6'),
            ('`', '¨'),
            ('{', 'Ú'),
            ('}', '('),
            ('~', '`'),
        ],
        "com.apple.keylayout.Danish" => &[
            ('"', '^'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'æ'),
            ('\'', '¨'),
            ('\\', '\''),
            (']', 'ø'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Æ'),
            ('|', '*'),
            ('}', 'Ø'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Faroese" => &[
            ('"', 'Ø'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Æ'),
            (';', 'æ'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'å'),
            ('\'', 'ø'),
            ('\\', '\''),
            (']', 'ð'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Å'),
            ('|', '*'),
            ('}', 'Ð'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Finnish" => &[
            ('"', '^'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '\''),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.FinnishExtended" => &[
            ('"', 'ˆ'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '\''),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.FinnishSami-PC" => &[
            ('"', 'ˆ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '@'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.French" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('.', ';'),
            ('/', ':'),
            ('0', 'à'),
            ('1', '&'),
            ('2', 'é'),
            ('3', '"'),
            ('4', '\''),
            ('5', '('),
            ('6', '§'),
            ('7', 'è'),
            ('8', '!'),
            ('9', 'ç'),
            (':', '°'),
            (';', ')'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', '^'),
            ('\'', 'ù'),
            ('\\', '`'),
            (']', '$'),
            ('^', '6'),
            ('`', '<'),
            ('{', '¨'),
            ('|', '£'),
            ('}', '*'),
            ('~', '>'),
        ],
        "com.apple.keylayout.French-PC" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('-', ')'),
            ('.', ';'),
            ('/', ':'),
            ('0', 'à'),
            ('1', '&'),
            ('2', 'é'),
            ('3', '"'),
            ('4', '\''),
            ('5', '('),
            ('6', '-'),
            ('7', 'è'),
            ('8', '_'),
            ('9', 'ç'),
            (':', '§'),
            (';', '!'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', '^'),
            ('\'', 'ù'),
            ('\\', '*'),
            (']', '$'),
            ('^', '6'),
            ('_', '°'),
            ('`', '<'),
            ('{', '¨'),
            ('|', 'μ'),
            ('}', '£'),
            ('~', '>'),
        ],
        "com.apple.keylayout.French-numerical" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('.', ';'),
            ('/', ':'),
            ('0', 'à'),
            ('1', '&'),
            ('2', 'é'),
            ('3', '"'),
            ('4', '\''),
            ('5', '('),
            ('6', '§'),
            ('7', 'è'),
            ('8', '!'),
            ('9', 'ç'),
            (':', '°'),
            (';', ')'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', '^'),
            ('\'', 'ù'),
            ('\\', '`'),
            (']', '$'),
            ('^', '6'),
            ('`', '<'),
            ('{', '¨'),
            ('|', '£'),
            ('}', '*'),
            ('~', '>'),
        ],
        "com.apple.keylayout.German" => &[
            ('"', '`'),
            ('#', '§'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', 'ß'),
            (':', 'Ü'),
            (';', 'ü'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '´'),
            ('\\', '#'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '\''),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.German-DIN-2137" => &[
            ('"', '`'),
            ('#', '§'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', 'ß'),
            (':', 'Ü'),
            (';', 'ü'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '´'),
            ('\\', '#'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '\''),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Hawaiian" => &[('\'', 'ʻ')],
        "com.apple.keylayout.Hungarian" => &[
            ('!', '\''),
            ('"', 'Á'),
            ('#', '+'),
            ('$', '!'),
            ('&', '='),
            ('(', ')'),
            (')', 'Ö'),
            ('*', '('),
            ('+', 'Ó'),
            ('/', 'ü'),
            ('0', 'ö'),
            (':', 'É'),
            (';', 'é'),
            ('<', 'Ü'),
            ('=', 'ó'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ő'),
            ('\'', 'á'),
            ('\\', 'ű'),
            (']', 'ú'),
            ('^', '/'),
            ('`', 'í'),
            ('{', 'Ő'),
            ('|', 'Ű'),
            ('}', 'Ú'),
            ('~', 'Í'),
        ],
        "com.apple.keylayout.Hungarian-QWERTY" => &[
            ('!', '\''),
            ('"', 'Á'),
            ('#', '+'),
            ('$', '!'),
            ('&', '='),
            ('(', ')'),
            (')', 'Ö'),
            ('*', '('),
            ('+', 'Ó'),
            ('/', 'ü'),
            ('0', 'ö'),
            (':', 'É'),
            (';', 'é'),
            ('<', 'Ü'),
            ('=', 'ó'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ő'),
            ('\'', 'á'),
            ('\\', 'ű'),
            (']', 'ú'),
            ('^', '/'),
            ('`', 'í'),
            ('{', 'Ő'),
            ('|', 'Ű'),
            ('}', 'Ú'),
            ('~', 'Í'),
        ],
        "com.apple.keylayout.Icelandic" => &[
            ('"', 'Ö'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '\''),
            (':', 'Ð'),
            (';', 'ð'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'æ'),
            ('\'', 'ö'),
            ('\\', 'þ'),
            (']', '´'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Æ'),
            ('|', 'Þ'),
            ('}', '´'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Irish" => &[('#', '£')],
        "com.apple.keylayout.IrishExtended" => &[('#', '£')],
        "com.apple.keylayout.Italian" => &[
            ('!', '1'),
            ('"', '%'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            (',', ';'),
            ('.', ':'),
            ('/', ','),
            ('0', 'é'),
            ('1', '&'),
            ('2', '"'),
            ('3', '\''),
            ('4', '('),
            ('5', 'ç'),
            ('6', 'è'),
            ('7', ')'),
            ('8', '£'),
            ('9', 'à'),
            (':', '!'),
            (';', 'ò'),
            ('<', '.'),
            ('>', '/'),
            ('@', '2'),
            ('[', 'ì'),
            ('\'', 'ù'),
            ('\\', '§'),
            (']', '$'),
            ('^', '6'),
            ('`', '<'),
            ('{', '^'),
            ('|', '°'),
            ('}', '*'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Italian-Pro" => &[
            ('"', '^'),
            ('#', '£'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '\''),
            (':', 'é'),
            (';', 'è'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ò'),
            ('\'', 'ì'),
            ('\\', 'ù'),
            (']', 'à'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'ç'),
            ('|', '§'),
            ('}', '°'),
            ('~', '>'),
        ],
        "com.apple.keylayout.LatinAmerican" => &[
            ('"', '¨'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '\''),
            (':', 'Ñ'),
            (';', 'ñ'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', '{'),
            ('\'', '´'),
            ('\\', '¿'),
            (']', '}'),
            ('^', '&'),
            ('`', '<'),
            ('{', '['),
            ('|', '¡'),
            ('}', ']'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Lithuanian" => &[
            ('!', 'Ą'),
            ('#', 'Ę'),
            ('$', 'Ė'),
            ('%', 'Į'),
            ('&', 'Ų'),
            ('*', 'Ū'),
            ('+', 'Ž'),
            ('1', 'ą'),
            ('2', 'č'),
            ('3', 'ę'),
            ('4', 'ė'),
            ('5', 'į'),
            ('6', 'š'),
            ('7', 'ų'),
            ('8', 'ū'),
            ('=', 'ž'),
            ('@', 'Č'),
            ('^', 'Š'),
        ],
        "com.apple.keylayout.Maltese" => &[
            ('#', '£'),
            ('[', 'ġ'),
            (']', 'ħ'),
            ('`', 'ż'),
            ('{', 'Ġ'),
            ('}', 'Ħ'),
            ('~', 'Ż'),
        ],
        "com.apple.keylayout.NorthernSami" => &[
            ('"', 'Ŋ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('Q', 'Á'),
            ('W', 'Š'),
            ('X', 'Č'),
            ('[', 'ø'),
            ('\'', 'ŋ'),
            ('\\', 'đ'),
            (']', 'æ'),
            ('^', '&'),
            ('`', 'ž'),
            ('q', 'á'),
            ('w', 'š'),
            ('x', 'č'),
            ('{', 'Ø'),
            ('|', 'Đ'),
            ('}', 'Æ'),
            ('~', 'Ž'),
        ],
        "com.apple.keylayout.Norwegian" => &[
            ('"', '^'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ø'),
            ('\'', '¨'),
            ('\\', '@'),
            (']', 'æ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ø'),
            ('|', '*'),
            ('}', 'Æ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.NorwegianExtended" => &[
            ('"', 'ˆ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ø'),
            ('\\', '@'),
            (']', 'æ'),
            ('`', '<'),
            ('}', 'Æ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.NorwegianSami-PC" => &[
            ('"', 'ˆ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ø'),
            ('\'', '¨'),
            ('\\', '@'),
            (']', 'æ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ø'),
            ('|', '*'),
            ('}', 'Æ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Polish" => &[
            ('!', '§'),
            ('"', 'ę'),
            ('#', '!'),
            ('$', '?'),
            ('%', '+'),
            ('&', ':'),
            ('(', '/'),
            (')', '"'),
            ('*', '_'),
            ('+', ']'),
            (',', '.'),
            ('.', ','),
            ('/', 'ż'),
            (':', 'Ł'),
            (';', 'ł'),
            ('<', 'ś'),
            ('=', '['),
            ('>', 'ń'),
            ('?', 'Ż'),
            ('@', '%'),
            ('[', 'ó'),
            ('\'', 'ą'),
            ('\\', ';'),
            (']', '('),
            ('^', '='),
            ('_', 'ć'),
            ('`', '<'),
            ('{', 'ź'),
            ('|', '$'),
            ('}', ')'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Portuguese" => &[
            ('"', '`'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '\''),
            (':', 'ª'),
            (';', 'º'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ç'),
            ('\'', '´'),
            (']', '~'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ç'),
            ('}', '^'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Sami-PC" => &[
            ('"', 'Ŋ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('Q', 'Á'),
            ('W', 'Š'),
            ('X', 'Č'),
            ('[', 'ø'),
            ('\'', 'ŋ'),
            ('\\', 'đ'),
            (']', 'æ'),
            ('^', '&'),
            ('`', 'ž'),
            ('q', 'á'),
            ('w', 'š'),
            ('x', 'č'),
            ('{', 'Ø'),
            ('|', 'Đ'),
            ('}', 'Æ'),
            ('~', 'Ž'),
        ],
        "com.apple.keylayout.Serbian-Latin" => &[
            ('"', 'Ć'),
            ('&', '\''),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            (':', 'Č'),
            (';', 'č'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'š'),
            ('\'', 'ć'),
            ('\\', 'ž'),
            (']', 'đ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Š'),
            ('|', 'Ž'),
            ('}', 'Đ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Slovak" => &[
            ('!', '1'),
            ('"', '!'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('+', '%'),
            ('/', '\''),
            ('0', 'é'),
            ('1', '+'),
            ('2', 'ľ'),
            ('3', 'š'),
            ('4', 'č'),
            ('5', 'ť'),
            ('6', 'ž'),
            ('7', 'ý'),
            ('8', 'á'),
            ('9', 'í'),
            (':', '"'),
            (';', 'ô'),
            ('<', '?'),
            ('>', ':'),
            ('?', 'ˇ'),
            ('@', '2'),
            ('[', 'ú'),
            ('\'', '§'),
            (']', 'ä'),
            ('^', '6'),
            ('`', 'ň'),
            ('{', 'Ú'),
            ('}', 'Ä'),
            ('~', 'Ň'),
        ],
        "com.apple.keylayout.Slovak-QWERTY" => &[
            ('!', '1'),
            ('"', '!'),
            ('#', '3'),
            ('$', '4'),
            ('%', '5'),
            ('&', '7'),
            ('(', '9'),
            (')', '0'),
            ('*', '8'),
            ('+', '%'),
            ('/', '\''),
            ('0', 'é'),
            ('1', '+'),
            ('2', 'ľ'),
            ('3', 'š'),
            ('4', 'č'),
            ('5', 'ť'),
            ('6', 'ž'),
            ('7', 'ý'),
            ('8', 'á'),
            ('9', 'í'),
            (':', '"'),
            (';', 'ô'),
            ('<', '?'),
            ('>', ':'),
            ('?', 'ˇ'),
            ('@', '2'),
            ('[', 'ú'),
            ('\'', '§'),
            (']', 'ä'),
            ('^', '6'),
            ('`', 'ň'),
            ('{', 'Ú'),
            ('}', 'Ä'),
            ('~', 'Ň'),
        ],
        "com.apple.keylayout.Slovenian" => &[
            ('"', 'Ć'),
            ('&', '\''),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            (':', 'Č'),
            (';', 'č'),
            ('<', ';'),
            ('=', '*'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'š'),
            ('\'', 'ć'),
            ('\\', 'ž'),
            (']', 'đ'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Š'),
            ('|', 'Ž'),
            ('}', 'Đ'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Spanish" => &[
            ('!', '¡'),
            ('"', '¨'),
            ('.', 'ç'),
            ('/', '.'),
            (':', 'º'),
            (';', '´'),
            ('<', '¿'),
            ('>', 'Ç'),
            ('@', '!'),
            ('[', 'ñ'),
            ('\'', '`'),
            ('\\', '\''),
            (']', ';'),
            ('^', '/'),
            ('`', '<'),
            ('{', 'Ñ'),
            ('|', '"'),
            ('}', ':'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Spanish-ISO" => &[
            ('"', '¨'),
            ('#', '·'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('.', 'ç'),
            ('/', '.'),
            (':', 'º'),
            (';', '´'),
            ('<', '¿'),
            ('>', 'Ç'),
            ('@', '"'),
            ('[', 'ñ'),
            ('\'', '`'),
            ('\\', '\''),
            (']', ';'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ñ'),
            ('|', '"'),
            ('}', '`'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Swedish" => &[
            ('"', '^'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '\''),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Swedish-Pro" => &[
            ('"', '^'),
            ('$', '€'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '\''),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.SwedishSami-PC" => &[
            ('"', 'ˆ'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('/', '´'),
            (':', 'Å'),
            (';', 'å'),
            ('<', ';'),
            ('=', '`'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '¨'),
            ('\\', '@'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ö'),
            ('|', '*'),
            ('}', 'Ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.SwissFrench" => &[
            ('!', '+'),
            ('"', '`'),
            ('#', '*'),
            ('$', 'ç'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('+', '!'),
            ('/', '\''),
            (':', 'ü'),
            (';', 'è'),
            ('<', ';'),
            ('=', '¨'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'é'),
            ('\'', '^'),
            ('\\', '$'),
            (']', 'à'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'ö'),
            ('|', '£'),
            ('}', 'ä'),
            ('~', '>'),
        ],
        "com.apple.keylayout.SwissGerman" => &[
            ('!', '+'),
            ('"', '`'),
            ('#', '*'),
            ('$', 'ç'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('+', '!'),
            ('/', '\''),
            (':', 'è'),
            (';', 'ü'),
            ('<', ';'),
            ('=', '¨'),
            ('>', ':'),
            ('@', '"'),
            ('[', 'ö'),
            ('\'', '^'),
            ('\\', '$'),
            (']', 'ä'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'é'),
            ('|', '£'),
            ('}', 'à'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Turkish" => &[
            ('"', '-'),
            ('#', '"'),
            ('$', '\''),
            ('%', '('),
            ('&', ')'),
            ('(', '%'),
            (')', ':'),
            ('*', '_'),
            (',', 'ö'),
            ('-', 'ş'),
            ('.', 'ç'),
            ('/', '.'),
            (':', '$'),
            ('<', 'Ö'),
            ('>', 'Ç'),
            ('@', '*'),
            ('[', 'ğ'),
            ('\'', ','),
            ('\\', 'ü'),
            (']', 'ı'),
            ('^', '/'),
            ('_', 'Ş'),
            ('`', '<'),
            ('{', 'Ğ'),
            ('|', 'Ü'),
            ('}', 'I'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Turkish-QWERTY-PC" => &[
            ('"', 'I'),
            ('#', '^'),
            ('$', '+'),
            ('&', '/'),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            ('+', ':'),
            (',', 'ö'),
            ('.', 'ç'),
            ('/', '*'),
            (':', 'Ş'),
            (';', 'ş'),
            ('<', 'Ö'),
            ('=', '.'),
            ('>', 'Ç'),
            ('@', '\''),
            ('[', 'ğ'),
            ('\'', 'ı'),
            ('\\', ','),
            (']', 'ü'),
            ('^', '&'),
            ('`', '<'),
            ('{', 'Ğ'),
            ('|', ';'),
            ('}', 'Ü'),
            ('~', '>'),
        ],
        "com.apple.keylayout.Turkish-Standard" => &[
            ('"', 'Ş'),
            ('#', '^'),
            ('&', '\''),
            ('(', ')'),
            (')', '='),
            ('*', '('),
            (',', '.'),
            ('.', ','),
            (':', 'Ç'),
            (';', 'ç'),
            ('<', ':'),
            ('=', '*'),
            ('>', ';'),
            ('@', '"'),
            ('[', 'ğ'),
            ('\'', 'ş'),
            ('\\', 'ü'),
            (']', 'ı'),
            ('^', '&'),
            ('`', 'ö'),
            ('{', 'Ğ'),
            ('|', 'Ü'),
            ('}', 'I'),
            ('~', 'Ö'),
        ],
        "com.apple.keylayout.Turkmen" => &[
            ('C', 'Ç'),
            ('Q', 'Ä'),
            ('V', 'Ý'),
            ('X', 'Ü'),
            ('[', 'ň'),
            ('\\', 'ş'),
            (']', 'ö'),
            ('^', '№'),
            ('`', 'ž'),
            ('c', 'ç'),
            ('q', 'ä'),
            ('v', 'ý'),
            ('x', 'ü'),
            ('{', 'Ň'),
            ('|', 'Ş'),
            ('}', 'Ö'),
            ('~', 'Ž'),
        ],
        "com.apple.keylayout.USInternational-PC" => &[('^', 'ˆ'), ('~', '˜')],
        "com.apple.keylayout.Welsh" => &[('#', '£')],

        _ => return None,
    };

    Some(HashMap::from_iter(mappings.into_iter().cloned()))
}

#[cfg(not(target_os = "macos"))]
pub fn get_key_equivalents(_layout: &str) -> Option<HashMap<char, char>> {
    None
}
