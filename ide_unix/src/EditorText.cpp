#include "./../include/EditorText.h"
#include "./../include/prefs.h"
#include <wx/wx.h>
//! wxWidgets headers
#include "wx/file.h"     // raw file io support
#include "wx/filename.h" // filename support
const int ANNOTATION_STYLE = wxSTC_STYLE_LASTPREDEFINED + 1;

//============================================================================
// implementation
//============================================================================

//----------------------------------------------------------------------------
// EditorText
//----------------------------------------------------------------------------

wxBEGIN_EVENT_TABLE (EditorText, wxStyledTextCtrl)
    // common
    EVT_SIZE (                         EditorText::OnSize)
    // edit
    EVT_MENU (wxID_CLEAR,              EditorText::OnEditClear)
    EVT_MENU (wxID_CUT,                EditorText::OnEditCut)
    EVT_MENU (wxID_COPY,               EditorText::OnEditCopy)
    EVT_MENU (wxID_PASTE,              EditorText::OnEditPaste)
    EVT_MENU (myID_INDENTINC,          EditorText::OnEditIndentInc)
    EVT_MENU (myID_INDENTRED,          EditorText::OnEditIndentRed)
    EVT_MENU (wxID_SELECTALL,          EditorText::OnEditSelectAll)
    EVT_MENU (myID_SELECTLINE,         EditorText::OnEditSelectLine)
    EVT_MENU (wxID_REDO,               EditorText::OnEditRedo)
    EVT_MENU (wxID_UNDO,               EditorText::OnEditUndo)
    // find
    EVT_MENU (wxID_FIND,               EditorText::OnFind)
    EVT_MENU (myID_FINDNEXT,           EditorText::OnFindNext)
    EVT_MENU (myID_REPLACE,            EditorText::OnReplace)
    EVT_MENU (myID_REPLACENEXT,        EditorText::OnReplaceNext)
    EVT_MENU (myID_BRACEMATCH,         EditorText::OnBraceMatch)
    EVT_MENU (myID_GOTO,               EditorText::OnGoto)
    // view
    EVT_MENU_RANGE (myID_HILIGHTFIRST, myID_HILIGHTLAST,
                                       EditorText::OnHilightLang)
    EVT_MENU (myID_DISPLAYEOL,         EditorText::OnDisplayEOL)
    EVT_MENU (myID_INDENTGUIDE,        EditorText::OnIndentGuide)
    EVT_MENU (myID_LINENUMBER,         EditorText::OnLineNumber)
    EVT_MENU (myID_LONGLINEON,         EditorText::OnLongLineOn)
    EVT_MENU (myID_WHITESPACE,         EditorText::OnWhiteSpace)
    EVT_MENU (myID_FOLDTOGGLE,         EditorText::OnFoldToggle)
    EVT_MENU (myID_OVERTYPE,           EditorText::OnSetOverType)
    EVT_MENU (myID_READONLY,           EditorText::OnSetReadOnly)
    EVT_MENU (myID_WRAPMODEON,         EditorText::OnWrapmodeOn)
    EVT_MENU (myID_CHARSETANSI,        EditorText::OnUseCharset)
    EVT_MENU (myID_CHARSETMAC,         EditorText::OnUseCharset)
    // annotations
    EVT_MENU (myID_ANNOTATION_ADD,     EditorText::OnAnnotationAdd)
    EVT_MENU (myID_ANNOTATION_REMOVE,  EditorText::OnAnnotationRemove)
    EVT_MENU (myID_ANNOTATION_CLEAR,   EditorText::OnAnnotationClear)
    EVT_MENU (myID_ANNOTATION_STYLE_HIDDEN,   EditorText::OnAnnotationStyle)
    EVT_MENU (myID_ANNOTATION_STYLE_STANDARD, EditorText::OnAnnotationStyle)
    EVT_MENU (myID_ANNOTATION_STYLE_BOXED,    EditorText::OnAnnotationStyle)
    // extra
    EVT_MENU (myID_CHANGELOWER,        EditorText::OnChangeCase)
    EVT_MENU (myID_CHANGEUPPER,        EditorText::OnChangeCase)
    EVT_MENU (myID_CONVERTCR,          EditorText::OnConvertEOL)
    EVT_MENU (myID_CONVERTCRLF,        EditorText::OnConvertEOL)
    EVT_MENU (myID_CONVERTLF,          EditorText::OnConvertEOL)
    // stc
    EVT_STC_MARGINCLICK (wxID_ANY,     EditorText::OnMarginClick)
    EVT_STC_CHARADDED (wxID_ANY,       EditorText::OnCharAdded)
    EVT_STC_KEY( wxID_ANY , EditorText::OnKey )

    EVT_KEY_DOWN( EditorText::OnKeyDown )
wxEND_EVENT_TABLE()



EditorText::EditorText(wxWindow* parent,
                        wxWindowID id,
                        const wxPoint &point,
                        const wxSize &size,
                        long style): wxStyledTextCtrl(parent, id, point, size, style)
{
    fileName = wxEmptyString;
    lineNrID = 0;
    dividerID = 1;
    foldingID = 2;

    language = NULL;
    SetStyleBits(8);

    SetViewEOL(gCommonPrefs.displayEOLEnable);
    SetIndentationGuides(gCommonPrefs.indentGuideEnable);

    SetEdgeMode(gCommonPrefs.longLineOnEnable ? wxSTC_EDGE_LINE : wxSTC_EDGE_NONE);
    SetViewWhiteSpace(gCommonPrefs.whiteSpaceEnable ? wxSTC_WS_VISIBLEALWAYS : wxSTC_WS_INVISIBLE);
    SetOvertype(gCommonPrefs.overTypeInitial);
    SetReadOnly(gCommonPrefs.readOnlyInitial);
    SetWrapMode(gCommonPrefs.wrapModeInitial ? wxSTC_WRAP_WORD : wxSTC_WRAP_NONE);

    wxFont font(10, wxMODERN, wxNORMAL, wxNORMAL);
    StyleSetFont(wxSTC_STYLE_DEFAULT, font);
    StyleSetForeground(wxSTC_STYLE_DEFAULT, *wxBLACK);
    StyleSetBackground(wxSTC_STYLE_DEFAULT, *wxWHITE);
    StyleSetForeground(wxSTC_STYLE_LINENUMBER, wxColour(wxT("DARK GREY")));
    StyleSetBackground(wxSTC_STYLE_LINENUMBER, *wxWHITE);
    StyleSetForeground(wxSTC_STYLE_INDENTGUIDE, wxColour(wxT("DARK GREY")));
    InitializePrefs(DEFAULT_LANGUAGE);

    SetVisiblePolicy(wxSTC_VISIBLE_STRICT|wxSTC_VISIBLE_SLOP, 1);
    SetXCaretPolicy(wxSTC_CARET_EVEN|wxSTC_VISIBLE_STRICT|wxSTC_VISIBLE_SLOP, 1);
    SetYCaretPolicy(wxSTC_CARET_EVEN|wxSTC_VISIBLE_STRICT|wxSTC_VISIBLE_SLOP, 1);

    // markers
    MarkerDefine (wxSTC_MARKNUM_FOLDER,        wxSTC_MARK_DOTDOTDOT, wxT("BLACK"), wxT("BLACK"));
    MarkerDefine (wxSTC_MARKNUM_FOLDEROPEN,    wxSTC_MARK_ARROWDOWN, wxT("BLACK"), wxT("BLACK"));
    MarkerDefine (wxSTC_MARKNUM_FOLDERSUB,     wxSTC_MARK_EMPTY,     wxT("BLACK"), wxT("BLACK"));
    MarkerDefine (wxSTC_MARKNUM_FOLDEREND,     wxSTC_MARK_DOTDOTDOT, wxT("BLACK"), wxT("WHITE"));
    MarkerDefine (wxSTC_MARKNUM_FOLDEROPENMID, wxSTC_MARK_ARROWDOWN, wxT("BLACK"), wxT("WHITE"));
    MarkerDefine (wxSTC_MARKNUM_FOLDERMIDTAIL, wxSTC_MARK_EMPTY,     wxT("BLACK"), wxT("BLACK"));
    MarkerDefine (wxSTC_MARKNUM_FOLDERTAIL,    wxSTC_MARK_EMPTY,     wxT("BLACK"), wxT("BLACK"));

    AnnotationSetVisible(wxSTC_ANNOTATION_BOXED);

    lineNrMargin = TextWidth(wxSTC_STYLE_LINENUMBER, wxT("_999999"));
    foldingMargin = 16;

    CmdKeyClear(wxSTC_KEY_TAB, 0);
    SetLayoutCache(wxSTC_CACHE_PAGE);

}

//----------------------------------------------------------------------------
// common event handlers
void EditorText::OnSize( wxSizeEvent& event ) {
    int x = GetClientSize().x +
            (gCommonPrefs.lineNumberEnable? lineNrMargin: 0) +
            (gCommonPrefs.foldEnable? foldingMargin: 0);
    if (x > 0) SetScrollWidth (x);
    event.Skip();
}

// edit event handlers
void EditorText::OnEditRedo (wxCommandEvent &WXUNUSED(event)) {
    if (!CanRedo()) return;
    Redo ();
}

void EditorText::OnEditUndo (wxCommandEvent &WXUNUSED(event)) {
    if (!CanUndo()) return;
    Undo ();
}

void EditorText::OnEditClear (wxCommandEvent &WXUNUSED(event)) {
    if (GetReadOnly()) return;
    Clear ();
}

void EditorText::OnKey (wxStyledTextEvent &WXUNUSED(event))
{
    wxMessageBox("OnKey");
}

void EditorText::OnKeyDown (wxKeyEvent &event)
{
    if (CallTipActive())
        CallTipCancel();
    if (event.GetKeyCode() == WXK_SPACE && event.ControlDown() && event.ShiftDown())
    {
        int pos = GetCurrentPos();
        CallTipSetBackground(*wxYELLOW);
        CallTipShow(pos,
                    "This is a CallTip with multiple lines.\n"
                    "It is meant to be a context sensitive popup helper for the user.");
        return;
    }
    event.Skip();
}

void EditorText::OnEditCut (wxCommandEvent &WXUNUSED(event)) {
    if (GetReadOnly() || (GetSelectionEnd()-GetSelectionStart() <= 0)) return;
    Cut ();
}

void EditorText::OnEditCopy (wxCommandEvent &WXUNUSED(event)) {
    if (GetSelectionEnd()-GetSelectionStart() <= 0) return;
    Copy ();
}

void EditorText::OnEditPaste (wxCommandEvent &WXUNUSED(event)) {
    if (!CanPaste()) return;
    Paste ();
}

void EditorText::OnFind (wxCommandEvent &WXUNUSED(event)) {
}

void EditorText::OnFindNext (wxCommandEvent &WXUNUSED(event)) {
}

void EditorText::OnReplace (wxCommandEvent &WXUNUSED(event)) {
}

void EditorText::OnReplaceNext (wxCommandEvent &WXUNUSED(event)) {
}

void EditorText::OnBraceMatch (wxCommandEvent &WXUNUSED(event)) {
    int min = GetCurrentPos ();
    int max = BraceMatch (min);
    if (max > (min+1)) {
        BraceHighlight (min+1, max);
        SetSelection (min+1, max);
    }else{
        BraceBadLight (min);
    }
}

void EditorText::OnGoto (wxCommandEvent &WXUNUSED(event)) {
}

void EditorText::OnEditIndentInc (wxCommandEvent &WXUNUSED(event)) {
    CmdKeyExecute (wxSTC_CMD_TAB);
}

void EditorText::OnEditIndentRed (wxCommandEvent &WXUNUSED(event)) {
    CmdKeyExecute (wxSTC_CMD_DELETEBACK);
}

void EditorText::OnEditSelectAll (wxCommandEvent &WXUNUSED(event)) {
    SetSelection (0, GetTextLength ());
}

void EditorText::OnEditSelectLine (wxCommandEvent &WXUNUSED(event)) {
    int lineStart = PositionFromLine (GetCurrentLine());
    int lineEnd = PositionFromLine (GetCurrentLine() + 1);
    SetSelection (lineStart, lineEnd);
}

void EditorText::OnHilightLang (wxCommandEvent &event) {
    InitializePrefs (gLanguagePrefs [event.GetId() - myID_HILIGHTFIRST].name);
}

void EditorText::OnDisplayEOL (wxCommandEvent &WXUNUSED(event)) {
    SetViewEOL (!GetViewEOL());
}

void EditorText::OnIndentGuide (wxCommandEvent &WXUNUSED(event)) {
    SetIndentationGuides (!GetIndentationGuides());
}

void EditorText::OnLineNumber (wxCommandEvent &WXUNUSED(event)) {
    SetMarginWidth (lineNrID,
                    GetMarginWidth (lineNrID) == 0? lineNrMargin: 0);
}

void EditorText::OnLongLineOn (wxCommandEvent &WXUNUSED(event)) {
    SetEdgeMode (GetEdgeMode() == 0? wxSTC_EDGE_LINE: wxSTC_EDGE_NONE);
}

void EditorText::OnWhiteSpace (wxCommandEvent &WXUNUSED(event)) {
    SetViewWhiteSpace (GetViewWhiteSpace() == 0?
                       wxSTC_WS_VISIBLEALWAYS: wxSTC_WS_INVISIBLE);
}

void EditorText::OnFoldToggle (wxCommandEvent &WXUNUSED(event)) {
    ToggleFold (GetFoldParent(GetCurrentLine()));
}

void EditorText::OnSetOverType (wxCommandEvent &WXUNUSED(event)) {
    SetOvertype (!GetOvertype());
}

void EditorText::OnSetReadOnly (wxCommandEvent &WXUNUSED(event)) {
    SetReadOnly (!GetReadOnly());
}

void EditorText::OnWrapmodeOn (wxCommandEvent &WXUNUSED(event)) {
    SetWrapMode (GetWrapMode() == 0? wxSTC_WRAP_WORD: wxSTC_WRAP_NONE);
}

void EditorText::OnUseCharset (wxCommandEvent &event) {
    int Nr;
    int charset = GetCodePage();
    switch (event.GetId()) {
        case myID_CHARSETANSI: {charset = wxSTC_CHARSET_ANSI; break;}
        case myID_CHARSETMAC: {charset = wxSTC_CHARSET_ANSI; break;}
    }
    for (Nr = 0; Nr < wxSTC_STYLE_LASTPREDEFINED; Nr++) {
        StyleSetCharacterSet (Nr, charset);
    }
    SetCodePage (charset);
}

void EditorText::OnAnnotationAdd(wxCommandEvent& WXUNUSED(event))
{
    const int line = GetCurrentLine();

    wxString ann = AnnotationGetText(line);
    ann = wxGetTextFromUser
          (
            wxString::Format("Enter annotation for the line %d", line),
            "Edit annotation",
            ann,
            this
          );
    if ( ann.empty() )
        return;

    AnnotationSetText(line, ann);
    AnnotationSetStyle(line, ANNOTATION_STYLE);

    // Scintilla doesn't update the scroll width for annotations, even with
    // scroll width tracking on, so do it manually.
    const int width = GetScrollWidth();

    // NB: The following adjustments are only needed when using
    //     wxSTC_ANNOTATION_BOXED annotations style, but we apply them always
    //     in order to make things simpler and not have to redo the width
    //     calculations when the annotations visibility changes. In a real
    //     program you'd either just stick to a fixed annotations visibility or
    //     update the width when it changes.

    // Take into account the fact that the annotation is shown indented, with
    // the same indent as the line it's attached to.
    int indent = GetLineIndentation(line);

    // This is just a hack to account for the width of the box, there doesn't
    // seem to be any way to get it directly from Scintilla.
    indent += 3;

    const int widthAnn = TextWidth(ANNOTATION_STYLE, ann + wxString(indent, ' '));

    if (widthAnn > width)
        SetScrollWidth(widthAnn);
}

void EditorText::OnAnnotationRemove(wxCommandEvent& WXUNUSED(event))
{
    AnnotationSetText(GetCurrentLine(), wxString());
}

void EditorText::OnAnnotationClear(wxCommandEvent& WXUNUSED(event))
{
    AnnotationClearAll();
}

void EditorText::OnAnnotationStyle(wxCommandEvent& event)
{
    int style = 0;
    switch (event.GetId()) {
        case myID_ANNOTATION_STYLE_HIDDEN:
            style = wxSTC_ANNOTATION_HIDDEN;
            break;

        case myID_ANNOTATION_STYLE_STANDARD:
            style = wxSTC_ANNOTATION_STANDARD;
            break;

        case myID_ANNOTATION_STYLE_BOXED:
            style = wxSTC_ANNOTATION_BOXED;
            break;
    }

    AnnotationSetVisible(style);
}

void EditorText::OnChangeCase (wxCommandEvent &event) {
    switch (event.GetId()) {
        case myID_CHANGELOWER: {
            CmdKeyExecute (wxSTC_CMD_LOWERCASE);
            break;
        }
        case myID_CHANGEUPPER: {
            CmdKeyExecute (wxSTC_CMD_UPPERCASE);
            break;
        }
    }
}

void EditorText::OnConvertEOL (wxCommandEvent &event) {
    int eolMode = GetEOLMode();
    switch (event.GetId()) {
        case myID_CONVERTCR: { eolMode = wxSTC_EOL_CR; break;}
        case myID_CONVERTCRLF: { eolMode = wxSTC_EOL_CRLF; break;}
        case myID_CONVERTLF: { eolMode = wxSTC_EOL_LF; break;}
    }
    ConvertEOLs (eolMode);
    SetEOLMode (eolMode);
}

//! misc
void EditorText::OnMarginClick (wxStyledTextEvent &event) {
    if (event.GetMargin() == 2) {
        int lineClick = LineFromPosition (event.GetPosition());
        int levelClick = GetFoldLevel (lineClick);
        if ((levelClick & wxSTC_FOLDLEVELHEADERFLAG) > 0) {
            ToggleFold (lineClick);
        }
    }
}

void EditorText::OnCharAdded (wxStyledTextEvent &event) {
    char chr = (char)event.GetKey();
    int currentLine = GetCurrentLine();
    // Change this if support for mac files with \r is needed
    if (chr == '\n') {
        int lineInd = 0;
        if (currentLine > 0) {
            lineInd = GetLineIndentation(currentLine - 1);
        }
        if (lineInd == 0) return;
        SetLineIndentation (currentLine, lineInd);
        GotoPos(PositionFromLine (currentLine) + lineInd);
    }
}

wxString EditorText::DeterminePrefs(const wxString &filename){

    LanguageInfo const* curInfo;

    // determine language from filepatterns
    int languageNr;
    for (languageNr = 0; languageNr < gLanguagePrefsSize; languageNr++) {
        curInfo = &gLanguagePrefs [languageNr];
        wxString filepattern = curInfo->filepattern;
        filepattern.Lower();
        while (!filepattern.empty()) {
            wxString cur = filepattern.BeforeFirst (';');
            if ((cur == filename) ||
                (cur == (filename.BeforeLast ('.') + wxT(".*"))) ||
                (cur == (wxT("*.") + filename.AfterLast ('.')))) {
                return curInfo->name;
            }
            filepattern = filepattern.AfterFirst (';');
        }
    }
    return wxEmptyString;

}

bool EditorText::InitializePrefs(const wxString &name){

    // initialize styles
    StyleClearAll();
    LanguageInfo const* curInfo = NULL;

    // determine language
    bool found = false;
    int languageNr;
    for (languageNr = 0; languageNr < gLanguagePrefsSize; languageNr++) {
        curInfo = &gLanguagePrefs [languageNr];
        if (curInfo->name == name) {
            found = true;
            break;
        }
    }

    if (!found) return false;

    // set lexer and language
    SetLexer (curInfo->lexer);
    language = curInfo;

    // set margin for line numbers
    SetMarginType (lineNrID, wxSTC_MARGIN_NUMBER);
    StyleSetForeground (wxSTC_STYLE_LINENUMBER, wxColour (wxT("DARK GREY")));
    StyleSetBackground (wxSTC_STYLE_LINENUMBER, *wxWHITE);
    SetMarginWidth (lineNrID, 0); // start out not visible

    // annotations style
    StyleSetBackground(ANNOTATION_STYLE, wxColour(244, 220, 220));
    StyleSetForeground(ANNOTATION_STYLE, *wxBLACK);
    StyleSetSizeFractional(ANNOTATION_STYLE,
            (StyleGetSizeFractional(wxSTC_STYLE_DEFAULT)*4)/5);

    // default fonts for all styles!
    int Nr;
    for (Nr = 0; Nr < wxSTC_STYLE_LASTPREDEFINED; Nr++) {
        wxFont font (10, wxMODERN, wxNORMAL, wxNORMAL);
        StyleSetFont (Nr, font);
    }

    // set common styles
    StyleSetForeground (wxSTC_STYLE_DEFAULT, wxColour (wxT("DARK GREY")));
    StyleSetForeground (wxSTC_STYLE_INDENTGUIDE, wxColour (wxT("DARK GREY")));

    // initialize settings
    if (gCommonPrefs.syntaxEnable) {
        int keywordnr = 0;
        for (Nr = 0; Nr < STYLE_TYPES_COUNT; Nr++) {
            if (curInfo->styles[Nr].type == -1) continue;
            const StyleInfo &curType = gStylePrefs [curInfo->styles[Nr].type];
            wxFont font (curType.fontsize, wxMODERN, wxNORMAL, wxNORMAL, false,
                         curType.fontname);
            StyleSetFont (Nr, font);
            if (curType.foreground) {
                StyleSetForeground (Nr, wxColour (curType.foreground));
            }
            if (curType.background) {
                StyleSetBackground (Nr, wxColour (curType.background));
            }
            StyleSetBold (Nr, (curType.fontstyle & STC_STYLE_BOLD) > 0);
            StyleSetItalic (Nr, (curType.fontstyle & STC_STYLE_ITALIC) > 0);
            StyleSetUnderline (Nr, (curType.fontstyle & STC_STYLE_UNDERL) > 0);
            StyleSetVisible (Nr, (curType.fontstyle & STC_STYLE_HIDDEN) == 0);
            StyleSetCase (Nr, curType.lettercase);
            const char *pwords = curInfo->styles[Nr].words;
            if (pwords) {
                SetKeyWords (keywordnr, pwords);
                keywordnr += 1;
            }
        }
    }

    // set margin as unused
    SetMarginType (dividerID, wxSTC_MARGIN_SYMBOL);
    SetMarginWidth (dividerID, 0);
    SetMarginSensitive (dividerID, false);

    // folding
    SetMarginType (foldingID, wxSTC_MARGIN_SYMBOL);
    SetMarginMask (foldingID, wxSTC_MASK_FOLDERS);
    StyleSetBackground (foldingID, *wxWHITE);
    SetMarginWidth (foldingID, 0);
    SetMarginSensitive (foldingID, false);
    if (gCommonPrefs.foldEnable) {
        SetMarginWidth (foldingID, curInfo->folds != 0? foldingMargin: 0);
        SetMarginSensitive (foldingID, curInfo->folds != 0);
        SetProperty (wxT("fold"), curInfo->folds != 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.comment"),
                     (curInfo->folds & STC_FOLD_COMMENT) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.compact"),
                     (curInfo->folds & STC_FOLD_COMPACT) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.preprocessor"),
                     (curInfo->folds & STC_FOLD_PREPROC) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.html"),
                     (curInfo->folds & STC_FOLD_HTML) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.html.preprocessor"),
                     (curInfo->folds & STC_FOLD_HTMLPREP) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.comment.python"),
                     (curInfo->folds & STC_FOLD_COMMENTPY) > 0? wxT("1"): wxT("0"));
        SetProperty (wxT("fold.quotes.python"),
                     (curInfo->folds & STC_FOLD_QUOTESPY) > 0? wxT("1"): wxT("0"));
    }
    SetFoldFlags (wxSTC_FOLDFLAG_LINEBEFORE_CONTRACTED |
                  wxSTC_FOLDFLAG_LINEAFTER_CONTRACTED);

    // set spaces and indention
    SetTabWidth (4);
    SetUseTabs (false);
    SetTabIndents (true);
    SetBackSpaceUnIndents (true);
    SetIndent (gCommonPrefs.indentEnable? 4: 0);

    // others
    SetViewEOL (gCommonPrefs.displayEOLEnable);
    SetIndentationGuides (gCommonPrefs.indentGuideEnable);
    SetEdgeColumn (80);
    SetEdgeMode (gCommonPrefs.longLineOnEnable? wxSTC_EDGE_LINE: wxSTC_EDGE_NONE);
    SetViewWhiteSpace (gCommonPrefs.whiteSpaceEnable?
                       wxSTC_WS_VISIBLEALWAYS: wxSTC_WS_INVISIBLE);
    SetOvertype (gCommonPrefs.overTypeInitial);
    SetReadOnly (gCommonPrefs.readOnlyInitial);
    SetWrapMode (gCommonPrefs.wrapModeInitial?
                 wxSTC_WRAP_WORD: wxSTC_WRAP_NONE);

    return true;
}

bool EditorText::LoadFile(){
#if wxUSE_FILEDLG
    // get filname
    if (!m_filename) {
        wxFileDialog dlg (this, wxT("Open file"), wxEmptyString, wxEmptyString,
                          wxT("Any file (*)|*"), wxFD_OPEN | wxFD_FILE_MUST_EXIST | wxFD_CHANGE_DIR);
        if (dlg.ShowModal() != wxID_OK) return false;
        m_filename = dlg.GetPath();
    }

    // load file
    return LoadFile (m_filename);
#else
    return false;
#endif // wxUSE_FILEDLG
}

bool EditorText::LoadFile(const wxString& filename){

    // load file in edit and clear undo
    if (!filename.empty()) fileName = filename;

    wxStyledTextCtrl::LoadFile(fileName);

    EmptyUndoBuffer();

    // determine lexer language
    wxFileName fname (fileName);
    InitializePrefs (DeterminePrefs (fname.GetFullName()));

    return true;
}

bool EditorText::SaveFile(){
    #if wxUSE_FILEDLG
    // return if no change
    if (!Modified()) return true;

    // get filname
    if (!m_filename) {
        wxFileDialog dlg (this, wxT("Save file"), wxEmptyString, wxEmptyString, wxT("Any file (*)|*"),
                          wxFD_SAVE | wxFD_OVERWRITE_PROMPT);
        if (dlg.ShowModal() != wxID_OK) return false;
        m_filename = dlg.GetPath();
    }

    // save file
    return SaveFile (m_filename);
#else
    return false;
#endif // wxUSE_FILEDLG
}

bool EditorText::SaveFile(const wxString &filename){

    // return if no change
    if (!Modified()) return true;

//     // save edit in file and clear undo
//     if (!filename.empty()) m_filename = filename;
//     wxFile file (m_filename, wxFile::write);
//     if (!file.IsOpened()) return false;
//     wxString buf = GetText();
//     bool okay = file.Write (buf);
//     file.Close();
//     if (!okay) return false;
//     EmptyUndoBuffer();
//     SetSavePoint();

//     return true;

    return wxStyledTextCtrl::SaveFile(filename);
}

bool EditorText::Modified(){
    //checks if the text file has been modified
    return(GetModify() && !GetReadOnly());
}

EditorText::~EditorText()
{
    //dtor
}


EditProperties::EditProperties (EditorText *edit,
                                long style)
        : wxDialog (edit, wxID_ANY, wxEmptyString,
                    wxDefaultPosition, wxDefaultSize,
                    style | wxDEFAULT_DIALOG_STYLE | wxRESIZE_BORDER) {

    // sets the application title
    SetTitle (_("Properties"));
    wxString text;

    // fullname
    wxBoxSizer *fullname = new wxBoxSizer (wxHORIZONTAL);
    fullname->Add (10, 0);
    fullname->Add (new wxStaticText (this, wxID_ANY, _("Full filename"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL);
    fullname->Add (new wxStaticText (this, wxID_ANY, edit->GetFilename()),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL);

    // text info
    wxGridSizer *textinfo = new wxGridSizer (4, 0, 2);
    textinfo->Add (new wxStaticText (this, wxID_ANY, _("Language"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    textinfo->Add (new wxStaticText (this, wxID_ANY, edit->language->name),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);
    textinfo->Add (new wxStaticText (this, wxID_ANY, _("Lexer-ID: "),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    text = wxString::Format (wxT("%d"), edit->GetLexer());
    textinfo->Add (new wxStaticText (this, wxID_ANY, text),
                   0, wxALIGN_RIGHT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);
    wxString EOLtype = wxEmptyString;
    switch (edit->GetEOLMode()) {
        case wxSTC_EOL_CR: {EOLtype = wxT("CR (Unix)"); break; }
        case wxSTC_EOL_CRLF: {EOLtype = wxT("CRLF (Windows)"); break; }
        case wxSTC_EOL_LF: {EOLtype = wxT("CR (Macintosh)"); break; }
    }
    textinfo->Add (new wxStaticText (this, wxID_ANY, _("Line endings"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    textinfo->Add (new wxStaticText (this, wxID_ANY, EOLtype),
                   0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);

    // text info box
    wxStaticBoxSizer *textinfos = new wxStaticBoxSizer (
                     new wxStaticBox (this, wxID_ANY, _("Informations")),
                     wxVERTICAL);
    textinfos->Add (textinfo, 0, wxEXPAND);
    textinfos->Add (0, 6);

    // statistic
    wxGridSizer *statistic = new wxGridSizer (4, 0, 2);
    statistic->Add (new wxStaticText (this, wxID_ANY, _("Total lines"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                    0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    text = wxString::Format (wxT("%d"), edit->GetLineCount());
    statistic->Add (new wxStaticText (this, wxID_ANY, text),
                    0, wxALIGN_RIGHT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);
    statistic->Add (new wxStaticText (this, wxID_ANY, _("Total chars"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                    0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    text = wxString::Format (wxT("%d"), edit->GetTextLength());
    statistic->Add (new wxStaticText (this, wxID_ANY, text),
                    0, wxALIGN_RIGHT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);
    statistic->Add (new wxStaticText (this, wxID_ANY, _("Current line"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                    0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    text = wxString::Format (wxT("%d"), edit->GetCurrentLine());
    statistic->Add (new wxStaticText (this, wxID_ANY, text),
                    0, wxALIGN_RIGHT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);
    statistic->Add (new wxStaticText (this, wxID_ANY, _("Current pos"),
                                     wxDefaultPosition, wxSize(80, wxDefaultCoord)),
                    0, wxALIGN_LEFT|wxALIGN_CENTER_VERTICAL|wxLEFT, 4);
    text = wxString::Format (wxT("%d"), edit->GetCurrentPos());
    statistic->Add (new wxStaticText (this, wxID_ANY, text),
                    0, wxALIGN_RIGHT|wxALIGN_CENTER_VERTICAL|wxRIGHT, 4);

    // char/line statistics
    wxStaticBoxSizer *statistics = new wxStaticBoxSizer (
                     new wxStaticBox (this, wxID_ANY, _("Statistics")),
                     wxVERTICAL);
    statistics->Add (statistic, 0, wxEXPAND);
    statistics->Add (0, 6);

    // total pane
    wxBoxSizer *totalpane = new wxBoxSizer (wxVERTICAL);
    totalpane->Add (fullname, 0, wxEXPAND | wxLEFT | wxRIGHT | wxTOP, 10);
    totalpane->Add (0, 6);
    totalpane->Add (textinfos, 0, wxEXPAND | wxLEFT | wxRIGHT, 10);
    totalpane->Add (0, 10);
    totalpane->Add (statistics, 0, wxEXPAND | wxLEFT | wxRIGHT, 10);
    totalpane->Add (0, 6);
    wxButton *okButton = new wxButton (this, wxID_OK, _("OK"));
    okButton->SetDefault();
    totalpane->Add (okButton, 0, wxALIGN_CENTER | wxALL, 10);

    SetSizerAndFit (totalpane);

    ShowModal();
}
