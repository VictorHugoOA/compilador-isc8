#include "prefs.h"

const CommonInfo gCommonPrefs = {
    true, //Syntax Enabled
    true, //Fold Enable
    true, //indentEnable

    //display prefs
    false, //overTypeInitial
    false, //readOnlyInitial
    false, //wrapModeInitial
    false, //displayEOLEnable
    false, //IndentGuideEnable
    true,  //LineNumbers Enable
    false, //longLineEnable
    false  //whiteSpaceEnable
};

//----------------------------------------------------------------------------
// keywordlists
// C++
const char* CppWordlist1 =
    "asm auto bool break case catch char class const const_cast "
    "continue default delete do double dynamic_cast else enum explicit "
    "export extern false float for friend goto if inline int long "
    "mutable namespace new operator private protected public register "
    "reinterpret_cast return short signed sizeof static static_cast "
    "struct switch template this throw true try typedef typeid "
    "typename union unsigned using virtual void volatile wchar_t "
    "while";
const char* CppWordlist2 =
    "file";
const char* CppWordlist3 =
    "a addindex addtogroup anchor arg attention author b brief bug c "
    "class code date def defgroup deprecated dontinclude e em endcode "
    "endhtmlonly endif endlatexonly endlink endverbatim enum example "
    "exception f$ f[ f] file fn hideinitializer htmlinclude "
    "htmlonly if image include ingroup internal invariant interface "
    "latexonly li line link mainpage name namespace nosubgrouping note "
    "overload p page par param post pre ref relates remarks return "
    "retval sa section see showinitializer since skip skipline struct "
    "subsection test throw todo typedef union until var verbatim "
    "verbinclude version warning weakgroup $ @ \"\" & < > # { }";
const char* CppWordError = "error failed warning";
/*
const char* TnyWordList1 = "bool "
                            "program else do until while repeat "
                            "if fi then "
                            "read write "
                            "not and or ";
*/
const char* TnyWordList1 = "bool float program else do until while repeat if fi then read write not and or ";

// Python
const char* PythonWordlist1 =
    "and assert break class continue def del elif else except exec "
    "finally for from global if import in is lambda None not or pass "
    "print raise return try while yield";
const char* PythonWordlist2 =
    "ACCELERATORS ALT AUTO3STATE AUTOCHECKBOX AUTORADIOBUTTON BEGIN "
    "BITMAP BLOCK BUTTON CAPTION CHARACTERISTICS CHECKBOX CLASS "
    "COMBOBOX CONTROL CTEXT CURSOR DEFPUSHBUTTON DIALOG DIALOGEX "
    "DISCARDABLE EDITTEXT END EXSTYLE FONT GROUPBOX ICON LANGUAGE "
    "LISTBOX LTEXT MENU MENUEX MENUITEM MESSAGETABLE POPUP PUSHBUTTON "
    "RADIOBUTTON RCDATA RTEXT SCROLLBAR SEPARATOR SHIFT STATE3 "
    "STRINGTABLE STYLE TEXTINCLUDE VALUE VERSION VERSIONINFO VIRTKEY";

    //----------------------------------------------------------------------------
//! languages
const LanguageInfo gLanguagePrefs [] = {
    // C++
    {"C++",
     "*.c;*.cc;*.cpp;*.cxx;*.cs;*.h;*.hh;*.hpp;*.hxx;*.sma",
     wxSTC_LEX_CPP,
     {{STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_COMMENT, NULL},
      {STC_TYPE_COMMENT_LINE, NULL},
      {STC_TYPE_COMMENT_DOC, NULL},
      {STC_TYPE_NUMBER, NULL},
      {STC_TYPE_WORD1, CppWordlist1}, // KEYWORDS
      {STC_TYPE_STRING, NULL},
      {STC_TYPE_CHARACTER, NULL},
      {STC_TYPE_UUID, NULL},
      {STC_TYPE_PREPROCESSOR, NULL},
      {STC_TYPE_OPERATOR, NULL},
      {STC_TYPE_IDENTIFIER, NULL},
      {STC_TYPE_STRING_EOL, NULL},
      {STC_TYPE_DEFAULT, NULL}, // VERBATIM
      {STC_TYPE_REGEX, NULL},
      {STC_TYPE_COMMENT_SPECIAL, NULL}, // DOXY
      {STC_TYPE_WORD2, CppWordlist2}, // EXTRA WORDS
      {STC_TYPE_WORD3, CppWordlist3}, // DOXY KEYWORDS
      {STC_TYPE_ERROR, CppWordError}, // KEYWORDS ERROR
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL}},
     STC_FOLD_COMMENT | STC_FOLD_COMPACT | STC_FOLD_PREPROC},
     {"Tiny",
     "*.tny;*.TNY",
     wxSTC_LEX_CONTAINER,
     {{STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_COMMENT, NULL},
      {STC_TYPE_COMMENT_LINE, NULL},
      {STC_TYPE_COMMENT_DOC, NULL},
      {STC_TYPE_NUMBER, NULL},
      {STC_TYPE_WORD1, TnyWordList1}, // KEYWORDS
      {STC_TYPE_STRING, NULL},
      {STC_TYPE_CHARACTER, NULL},
      {STC_TYPE_UUID, NULL},
      {STC_TYPE_PREPROCESSOR, NULL},
      {STC_TYPE_OPERATOR, NULL},
      {STC_TYPE_IDENTIFIER, NULL},
      {STC_TYPE_STRING_EOL, NULL},
      {STC_TYPE_DEFAULT, NULL}, // VERBATIM
      {STC_TYPE_REGEX, NULL},
      {STC_TYPE_COMMENT_SPECIAL, NULL}, // DOXY
      {STC_TYPE_WORD2, NULL}, // EXTRA WORDS
      {STC_TYPE_WORD3, NULL}, // DOXY KEYWORDS
      {STC_TYPE_ERROR, CppWordError}, // KEYWORDS ERROR
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL}},
     STC_FOLD_COMMENT | STC_FOLD_COMPACT | STC_FOLD_PREPROC},
    // Python
    {"Python",
     "*.py;*.pyw",
     wxSTC_LEX_PYTHON,
     {{STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_COMMENT_LINE, NULL},
      {STC_TYPE_NUMBER, NULL},
      {STC_TYPE_STRING, NULL},
      {STC_TYPE_CHARACTER, NULL},
      {STC_TYPE_WORD1, PythonWordlist1}, // KEYWORDS
      {STC_TYPE_DEFAULT, NULL}, // TRIPLE
      {STC_TYPE_DEFAULT, NULL}, // TRIPLEDOUBLE
      {STC_TYPE_DEFAULT, NULL}, // CLASSNAME
      {STC_TYPE_DEFAULT, PythonWordlist2}, // DEFNAME
      {STC_TYPE_OPERATOR, NULL},
      {STC_TYPE_IDENTIFIER, NULL},
      {STC_TYPE_DEFAULT, NULL}, // COMMENT_BLOCK
      {STC_TYPE_STRING_EOL, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL}},
     STC_FOLD_COMMENTPY | STC_FOLD_QUOTESPY},
    // * (any)
    {wxTRANSLATE(DEFAULT_LANGUAGE),
     "*.*",
     wxSTC_LEX_PROPERTIES,
     {{STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_DEFAULT, NULL},
      {STC_TYPE_DEFAULT, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL},
      {-1, NULL}},
     0}
};
//Global variable for size
const int gLanguagePrefsSize = WXSIZEOF(gLanguagePrefs);


//----------------------------------------------------------------------------
//! style types
const StyleInfo gStylePrefs [] = {
    // mySTC_TYPE_DEFAULT
    {wxT("Default"),
     wxT("BLACK"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_WORD1
    {wxT("Keyword1"),
     wxT("BLUE"), wxT("WHITE"),
     wxT(""), 10, STC_STYLE_BOLD, 0},

    // mySTC_TYPE_WORD2
    {wxT("Keyword2"),
     wxT("MIDNIGHT BLUE"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_WORD3
    {wxT("Keyword3"),
     wxT("CORNFLOWER BLUE"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_WORD4
    {wxT("Keyword4"),
     wxT("CYAN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_WORD5
    {wxT("Keyword5"),
     wxT("DARK GREY"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_WORD6
    {wxT("Keyword6"),
     wxT("GREY"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_COMMENT
    {wxT("Comment"),
     wxT("FOREST GREEN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_COMMENT_DOC
    {wxT("Comment (Doc)"),
     wxT("FOREST GREEN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_COMMENT_LINE
    {wxT("Comment line"),
     wxT("FOREST GREEN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_COMMENT_SPECIAL
    {wxT("Special comment"),
     wxT("FOREST GREEN"), wxT("WHITE"),
     wxT(""), 10, STC_STYLE_ITALIC, 0},

    // mySTC_TYPE_CHARACTER
    {wxT("Character"),
     wxT("KHAKI"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_CHARACTER_EOL
    {wxT("Character (EOL)"),
     wxT("KHAKI"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_STRING
    {wxT("String"),
     wxT("BROWN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_STRING_EOL
    {wxT("String (EOL)"),
     wxT("BROWN"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_DELIMITER
    {wxT("Delimiter"),
     wxT("ORANGE"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_PUNCTUATION
    {wxT("Punctuation"),
     wxT("ORANGE"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_OPERATOR
    {wxT("Operator"),
     wxT("BLACK"), wxT("WHITE"),
     wxT(""), 10, STC_STYLE_BOLD, 0},

    // mySTC_TYPE_BRACE
    {wxT("Label"),
     wxT("VIOLET"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_COMMAND
    {wxT("Command"),
     wxT("BLUE"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_IDENTIFIER
    {wxT("Identifier"),
     wxT("BLACK"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_LABEL
    {wxT("Label"),
     wxT("VIOLET"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_NUMBER
    {wxT("Number"),
     wxT("SIENNA"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_PARAMETER
    {wxT("Parameter"),
     wxT("VIOLET"), wxT("WHITE"),
     wxT(""), 10, STC_STYLE_ITALIC, 0},

    // mySTC_TYPE_REGEX
    {wxT("Regular expression"),
     wxT("ORCHID"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_UUID
    {wxT("UUID"),
     wxT("ORCHID"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_VALUE
    {wxT("Value"),
     wxT("ORCHID"), wxT("WHITE"),
     wxT(""), 10, STC_STYLE_ITALIC, 0},

    // mySTC_TYPE_PREPROCESSOR
    {wxT("Preprocessor"),
     wxT("GREY"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_SCRIPT
    {wxT("Script"),
     wxT("DARK GREY"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_ERROR
    {wxT("Error"),
     wxT("RED"), wxT("WHITE"),
     wxT(""), 10, 0, 0},

    // mySTC_TYPE_UNDEFINED
    {wxT("Undefined"),
     wxT("ORANGE"), wxT("WHITE"),
     wxT(""), 10, 0, 0}

    };
//Globar variable for define all styles
const int gStylePrefsSize = WXSIZEOF(gStylePrefs);
