#ifndef PREFS_H_INCLUDED
#define PREFS_H_INCLUDED

#include <wx/stc/stc.h>

//all of these are general style types
#define STC_TYPE_DEFAULT 0
#define STC_TYPE_WORD1 1
#define STC_TYPE_WORD2 2
#define STC_TYPE_WORD3 3
#define STC_TYPE_WORD4 4
#define STC_TYPE_WORD5 5
#define STC_TYPE_WORD6 6

#define STC_TYPE_COMMENT 7
#define STC_TYPE_COMMENT_DOC 8
#define STC_TYPE_COMMENT_LINE 9
#define STC_TYPE_COMMENT_SPECIAL 10

#define STC_TYPE_CHARACTER 11
#define STC_TYPE_CHARACTER_EOL 12
#define STC_TYPE_STRING 13
#define STC_TYPE_STRING_EOL 14

#define STC_TYPE_DELIMITER 15
#define STC_TYPE_PUNCTUATION 16
#define STC_TYPE_OPERATOR 17
#define STC_TYPE_BRACE 18
#define STC_TYPE_COMMAND 19
#define STC_TYPE_IDENTIFIER 20
#define STC_TYPE_LABEL 21
#define STC_TYPE_NUMBER 22
#define STC_TYPE_PARAMETER 23
#define STC_TYPE_REGEX 24
#define STC_TYPE_UUID 25
#define STC_TYPE_VALUE 26
#define STC_TYPE_PREPROCESSOR 27
#define STC_TYPE_SCRIPT 28
#define STC_TYPE_ERROR 29

//style bits types
#define STC_STYLE_BOLD 1
#define STC_STYLE_ITALIC 2
#define STC_STYLE_UNDERL 4
#define STC_STYLE_HIDDEN 8

//general folding types
#define STC_FOLD_COMMENT 1
#define STC_FOLD_COMPACT 2
#define STC_FOLD_PREPROC 4
#define STC_FOLD_HTML 16
#define STC_FOLD_HTMLPREP 32
#define STC_FOLD_COMMENTPY 64
#define STC_FOLD_QUOTESPY 128

//flags
#define STC_FLAG_WRAPMODE 16

//declarations
#define DEFAULT_LANGUAGE "<default>"
#define PAGE_COMMON _("Common")
#define PAGE_LANGUAGES _("Languages")
#define PAGE_STYLE_TYPES _("Style types")
#define STYLE_TYPES_COUNT 32

//Common properties for editor.
struct CommonInfo{
    bool syntaxEnable;
    bool foldEnable;
    bool indentEnable;
    bool readOnlyInitial;
    bool overTypeInitial;
    bool wrapModeInitial;
    bool displayEOLEnable;
    bool indentGuideEnable;
    bool lineNumberEnable;
    bool longLineOnEnable;
    bool whiteSpaceEnable;
};
//Common properties used global
extern const CommonInfo gCommonPrefs;

//Struct that declares all info necesary for the language of the file
struct LanguageInfo{
    const char* name;
    const char* filepattern;
    int lexer;
    struct{
        int type;
        const char *words;
    } styles[STYLE_TYPES_COUNT];
    int folds;
};

extern const LanguageInfo gLanguagePrefs[];
extern const int gLanguagePrefsSize;

//Text editor changes for personal use
struct StyleInfo{
    const wxChar* name;
    const wxChar* foreground;
    const wxChar* background;
    const wxChar* fontname;
    int fontsize;
    int fontstyle;
    int lettercase;
};

extern const StyleInfo gStylePrefs[];
extern const int gStylePrefsSize;


enum {
    // menu IDs
    myID_PROPERTIES = wxID_HIGHEST,
    myID_EDIT_FIRST,
    myID_INDENTINC = myID_EDIT_FIRST,
    myID_INDENTRED,
    myID_FINDNEXT,
    myID_REPLACE,
    myID_REPLACENEXT,
    myID_BRACEMATCH,
    myID_GOTO,
    myID_PAGEACTIVE,
    myID_DISPLAYEOL,
    myID_INDENTGUIDE,
    myID_LINENUMBER,
    myID_LONGLINEON,
    myID_WHITESPACE,
    myID_FOLDTOGGLE,
    myID_OVERTYPE,
    myID_READONLY,
    myID_WRAPMODEON,
    myID_ANNOTATION_ADD,
    myID_ANNOTATION_REMOVE,
    myID_ANNOTATION_CLEAR,
    myID_ANNOTATION_STYLE_HIDDEN,
    myID_ANNOTATION_STYLE_STANDARD,
    myID_ANNOTATION_STYLE_BOXED,
    myID_CHANGECASE,
    myID_CHANGELOWER,
    myID_CHANGEUPPER,
    myID_HILIGHTLANG,
    myID_HILIGHTFIRST,
    myID_HILIGHTLAST = myID_HILIGHTFIRST + 99,
    myID_CONVERTEOL,
    myID_CONVERTCR,
    myID_CONVERTCRLF,
    myID_CONVERTLF,
    myID_USECHARSET,
    myID_CHARSETANSI,
    myID_CHARSETMAC,
    myID_PAGEPREV,
    myID_PAGENEXT,
    myID_SELECTLINE,
    myID_EDIT_LAST = myID_SELECTLINE,
    myID_WINDOW_MINIMAL,

    // other IDs
    myID_STATUSBAR,
    myID_TITLEBAR,
    myID_ABOUTTIMER,
    myID_UPDATETIMER,

    // dialog find IDs
    myID_DLG_FIND_TEXT,

    // preferences IDs
    myID_PREFS_LANGUAGE,
    myID_PREFS_STYLETYPE,
    myID_PREFS_KEYWORDS,
    ID_COMPILE_EVENT
};



#endif // PREFS_H_INCLUDED
