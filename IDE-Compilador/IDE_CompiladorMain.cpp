/***************************************************************
 * Name:      IDE_CompiladorMain.cpp
 * Purpose:   Code for Application Frame
 * Author:     ()
 * Created:   2021-02-08
 * Copyright:  ()
 * License:
 **************************************************************/

#include "IDE_CompiladorMain.h"
#include <wx/msgdlg.h>


#include <wx/intl.h>
#include <wx/string.h>
#include <wx/process.h>
#include <wx/txtstrm.h>
#include "ResultsShow.h"
#include "CompilerPhases.h"
#include <stdlib.h>

#define LEXIC_PATH "lexic-analyzer.exe "

//helper functions
enum wxbuildinfoformat {
    short_f, long_f };

wxString wxbuildinfo(wxbuildinfoformat format)
{
    wxString wxbuild(wxVERSION_STRING);

    if (format == long_f )
    {
#if defined(__WXMSW__)
        wxbuild << _T("-Windows");
#elif defined(__UNIX__)
        wxbuild << _T("-Linux");
#endif

#if wxUSE_UNICODE
        wxbuild << _T("-Unicode build");
#else
        wxbuild << _T("-ANSI build");
#endif // wxUSE_UNICODE
    }

    return wxbuild;
}
//All event tables to function respond on menu
wxBEGIN_EVENT_TABLE (IDE_CompiladorFrame, wxFrame)
    // common
    EVT_CLOSE (                      IDE_CompiladorFrame::OnClose)
    // file
    EVT_MENU (wxID_OPEN,             IDE_CompiladorFrame::OnFileOpen)
    EVT_MENU (wxID_SAVE,             IDE_CompiladorFrame::OnFileSave)
    EVT_MENU (wxID_SAVEAS,           IDE_CompiladorFrame::OnFileSaveAs)
    EVT_MENU (wxID_CLOSE,            IDE_CompiladorFrame::OnFileClose)
    // properties
    EVT_MENU (myID_PROPERTIES,       IDE_CompiladorFrame::OnProperties)
    EVT_MENU (wxID_EXIT,             IDE_CompiladorFrame::OnExit)
    // Menu items with standard IDs forwarded to the editor.
    EVT_MENU (wxID_CLEAR,            IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_CUT,              IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_COPY,             IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_PASTE,            IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_SELECTALL,        IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_REDO,             IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_UNDO,             IDE_CompiladorFrame::OnEdit)
    EVT_MENU (wxID_FIND,             IDE_CompiladorFrame::OnEdit)
    // And all our edit-related menu commands.
    EVT_MENU_RANGE (myID_EDIT_FIRST, myID_EDIT_LAST,
                                     IDE_CompiladorFrame::OnEdit)
    // help
    EVT_MENU (wxID_ABOUT,            IDE_CompiladorFrame::OnAbout)
    EVT_MENU (ID_COMPILE_EVENT,      IDE_CompiladorFrame::OnCompile)
wxEND_EVENT_TABLE ()

IDE_CompiladorFrame::IDE_CompiladorFrame(wxWindow* parent,wxWindowID id): wxFrame(parent, id, "Compilador ISC 8º", wxDefaultPosition, wxSize(800, 600))
{
    //sets the backgorund of the frame
    SetBackgroundColour(wxT("WHITE"));

    //creates the menubar
    menuBar = new wxMenuBar;

    //function to add all menu options
    CreateMenu();

    vertical = new wxBoxSizer(wxVERTICAL);
    horizontal = new wxBoxSizer(wxHORIZONTAL);
    //Creates the text editor and set it ot focus, so we can start writing
    editor = new EditorText(this, wxID_ANY);
    editor->SetFocus();
    //Creates the results panel
    results = new ResultsShow(this, wxID_ANY);
    phases = new CompilerPhases(this, wxID_ANY);
    horizontal->Add(editor, 1, wxALL|wxEXPAND, 0);
    horizontal->Add(phases, 1, wxALL|wxEXPAND, 0);

    //Add its to the vertical sizer so we can divide the windows
    vertical->Add(horizontal, 10, wxALL|wxEXPAND, 0);//All windows can expand to vertical full size and horizontal full size
    vertical->Add(results, 2, wxALL|wxEXPAND, 0);//The proportion for windows are 80% text editor 20% the results panel
    SetSizerAndFit(vertical);//set fit sizer with windows
    SetSize(wxDefaultCoord, wxDefaultCoord, 800, 600);//Set again size for the frame
}

void IDE_CompiladorFrame::OnClose(wxCloseEvent& event){
    wxCommandEvent evt;
    OnFileClose (evt);
    //Makes sure the file is saved
    if (editor && editor->Modified()) {
        if (event.CanVeto()) event.Veto (true);
        return;
    }
    //Destroy all widgets and exits program
    Destroy();
}

void IDE_CompiladorFrame::OnAbout (wxCommandEvent &WXUNUSED(event)) {
//WIP
}
//Closes the app
void IDE_CompiladorFrame::OnExit (wxCommandEvent &WXUNUSED(event)) {
    Close (true);
}

void IDE_CompiladorFrame::OnFileOpen (wxCommandEvent &WXUNUSED(event)) {
    if (!editor) return;
    //this is used in case wxUSE_FILEDLG is defined in compile editor
#if wxUSE_FILEDLG
    wxString fname;
    wxFileDialog dlg (this, wxT("Open file"), wxEmptyString, wxEmptyString, wxT("Any file (*)|*"),
                      wxFD_OPEN | wxFD_FILE_MUST_EXIST | wxFD_CHANGE_DIR);
    if (dlg.ShowModal() != wxID_OK) return;
    fname = dlg.GetPath ();
    FileOpen (fname);
#endif // wxUSE_FILEDLG
}

void IDE_CompiladorFrame::OnFileSave (wxCommandEvent &WXUNUSED(event)) {
    if (!editor) return;
    //if the text haven't been modified, message box to show
    if (!editor->Modified()) {
        //wxMessageBox (_("There is nothing to save!"), _("Save file"),
          //            wxOK | wxICON_EXCLAMATION);
        return;
    }
    editor->SaveFile ();
}

void IDE_CompiladorFrame::OnFileSaveAs (wxCommandEvent &WXUNUSED(event)) {
    if (!editor) return;
    //this is used in case wxUSE_FILEDLG is defined in compile editor
#if wxUSE_FILEDLG
    wxString filename = wxEmptyString;
    wxFileDialog dlg (this, wxT("Save file"), wxEmptyString, wxEmptyString, wxT("Any file (*)|*"), wxFD_SAVE|wxFD_OVERWRITE_PROMPT);
    if (dlg.ShowModal() != wxID_OK) return;
    filename = dlg.GetPath();
    editor->SaveFile (filename);
#endif // wxUSE_FILEDLG
}
//File on close
void IDE_CompiladorFrame::OnFileClose (wxCommandEvent &WXUNUSED(event)) {
    if (!editor) return;
    //Whenever the file has been modified, the text editor will ask to save it if you care.
    if (editor->Modified()) {
        if (wxMessageBox (_("Text is not saved, save before closing?"), _("Close"),
                          wxYES_NO | wxICON_QUESTION) == wxYES) {
            editor->SaveFile();
            //In case the text editor couldn't save the file
            if (editor->Modified()) {
                wxMessageBox (_("Text could not be saved!"), _("Close abort"),
                              wxOK | wxICON_EXCLAMATION);
                return;
            }
        }
    }
    editor->SetFilename (wxEmptyString);
    editor->ClearAll();
    editor->SetSavePoint();
}
//File properties dialog
void IDE_CompiladorFrame::OnProperties (wxCommandEvent &WXUNUSED(event)) {
    if (!editor) return;
    EditProperties dlg(editor, 0);
}
//Checks edit event to text editor;
void IDE_CompiladorFrame::OnEdit (wxCommandEvent &event) {
    if (editor) editor->GetEventHandler()->ProcessEvent (event);
}

void IDE_CompiladorFrame::CreateMenu ()
{
    // File menu
    wxMenu *menuFile = new wxMenu;
    menuFile->Append (wxID_OPEN, _("&Open ..\tCtrl+O"));
    menuFile->Append (wxID_SAVE, _("&Save\tCtrl+S"));
    menuFile->Append (wxID_SAVEAS, _("Save &as ..\tCtrl+Shift+S"));
    menuFile->Append (wxID_CLOSE, _("&Close\tCtrl+W"));
    menuFile->AppendSeparator();
    menuFile->Append (myID_PROPERTIES, _("Proper&ties ..\tCtrl+I"));
    menuFile->AppendSeparator();
    menuFile->Append (wxID_EXIT, _("&Quit\tCtrl+Q"));

    // Edit menu
    wxMenu *menuEdit = new wxMenu;
    menuEdit->Append (wxID_UNDO, _("&Undo\tCtrl+Z"));
    menuEdit->Append (wxID_REDO, _("&Redo\tCtrl+Shift+Z"));
    menuEdit->AppendSeparator();
    menuEdit->Append (wxID_CUT, _("Cu&t\tCtrl+X"));
    menuEdit->Append (wxID_COPY, _("&Copy\tCtrl+C"));
    menuEdit->Append (wxID_PASTE, _("&Paste\tCtrl+V"));
    menuEdit->Append (wxID_CLEAR, _("&Delete\tDel"));
    menuEdit->AppendSeparator();
    menuEdit->Append (wxID_FIND, _("&Find\tCtrl+F"));
    menuEdit->Enable (wxID_FIND, false);
    menuEdit->Append (myID_FINDNEXT, _("Find &next\tF3"));
    menuEdit->Enable (myID_FINDNEXT, false);
    menuEdit->Append (myID_REPLACE, _("&Replace\tCtrl+H"));
    menuEdit->Enable (myID_REPLACE, false);
    menuEdit->Append (myID_REPLACENEXT, _("Replace &again\tShift+F4"));
    menuEdit->Enable (myID_REPLACENEXT, false);
    menuEdit->AppendSeparator();
    menuEdit->Append (myID_BRACEMATCH, _("&Match brace\tCtrl+M"));
    menuEdit->Append (myID_GOTO, _("&Goto\tCtrl+G"));
    menuEdit->Enable (myID_GOTO, false);
    menuEdit->AppendSeparator();
    menuEdit->Append (myID_INDENTINC, _("&Indent increase\tTab"));
    menuEdit->Append (myID_INDENTRED, _("I&ndent reduce\tShift+Tab"));
    menuEdit->AppendSeparator();
    menuEdit->Append (wxID_SELECTALL, _("&Select all\tCtrl+A"));
    menuEdit->Append (myID_SELECTLINE, _("Select &line\tCtrl+L"));

    // hilight submenu
    wxMenu *menuHilight = new wxMenu;
    int Nr;
    for (Nr = 0; Nr < gLanguagePrefsSize; Nr++) {
        menuHilight->Append (myID_HILIGHTFIRST + Nr,
                             gLanguagePrefs [Nr].name);
    }

    // charset submenu
    wxMenu *menuCharset = new wxMenu;
    menuCharset->Append (myID_CHARSETANSI, _("&ANSI (Windows)"));
    menuCharset->Append (myID_CHARSETMAC, _("&MAC (Macintosh)"));

    // View menu
    wxMenu *menuView = new wxMenu;
    menuView->Append (myID_HILIGHTLANG, _("&Hilight language .."), menuHilight);
    menuView->AppendSeparator();
    menuView->AppendCheckItem (myID_FOLDTOGGLE, _("&Toggle current fold\tCtrl+T"));
    menuView->AppendCheckItem (myID_OVERTYPE, _("&Overwrite mode\tIns"));
    menuView->AppendCheckItem (myID_WRAPMODEON, _("&Wrap mode\tCtrl+U"));
    menuView->AppendSeparator();
    menuView->AppendCheckItem (myID_DISPLAYEOL, _("Show line &endings"));
    menuView->AppendCheckItem (myID_INDENTGUIDE, _("Show &indent guides"));
    menuView->AppendCheckItem (myID_LINENUMBER, _("Show line &numbers"));
    menuView->AppendCheckItem (myID_LONGLINEON, _("Show &long line marker"));
    menuView->AppendCheckItem (myID_WHITESPACE, _("Show white&space"));
    menuView->AppendSeparator();
    menuView->Append (myID_USECHARSET, _("Use &code page of .."), menuCharset);

    // Annotations menu
    wxMenu* menuAnnotations = new wxMenu;
    menuAnnotations->Append(myID_ANNOTATION_ADD, _("&Add or edit an annotation..."),
                            _("Add an annotation for the current line"));
    menuAnnotations->Append(myID_ANNOTATION_REMOVE, _("&Remove annotation"),
                            _("Remove the annotation for the current line"));
    menuAnnotations->Append(myID_ANNOTATION_CLEAR, _("&Clear all annotations"));

    wxMenu* menuAnnotationsStyle = new wxMenu;
    menuAnnotationsStyle->AppendRadioItem(myID_ANNOTATION_STYLE_HIDDEN, _("&Hidden"));
    menuAnnotationsStyle->AppendRadioItem(myID_ANNOTATION_STYLE_STANDARD, _("&Standard"));
    menuAnnotationsStyle->AppendRadioItem(myID_ANNOTATION_STYLE_BOXED, _("&Boxed"));
    menuAnnotations->AppendSubMenu(menuAnnotationsStyle, "&Style");

    // change case submenu
    wxMenu *menuChangeCase = new wxMenu;
    menuChangeCase->Append (myID_CHANGEUPPER, _("&Upper case"));
    menuChangeCase->Append (myID_CHANGELOWER, _("&Lower case"));

    // convert EOL submenu
    wxMenu *menuConvertEOL = new wxMenu;
    menuConvertEOL->Append (myID_CONVERTCR, _("CR (&Linux)"));
    menuConvertEOL->Append (myID_CONVERTCRLF, _("CR+LF (&Windows)"));
    menuConvertEOL->Append (myID_CONVERTLF, _("LF (&Macintosh)"));

    // Extra menu
    wxMenu *menuExtra = new wxMenu;
    menuExtra->AppendCheckItem (myID_READONLY, _("&Readonly mode"));
    menuExtra->AppendSeparator();
    menuExtra->Append (myID_CHANGECASE, _("Change &case to .."), menuChangeCase);
    menuExtra->AppendSeparator();
    menuExtra->Append (myID_CONVERTEOL, _("Convert line &endings to .."), menuConvertEOL);

    // Window menu
    wxMenu *menuWindow = new wxMenu;
    menuWindow->Append (myID_PAGEPREV, _("&Previous\tCtrl+Shift+Tab"));
    menuWindow->Append (myID_PAGENEXT, _("&Next\tCtrl+Tab"));

    // Help menu
    wxMenu *menuHelp = new wxMenu;
    menuHelp->Append (wxID_ABOUT, _("&About ..\tCtrl+D"));

    //Compile menu
    wxMenu *menuCompile = new wxMenu;
    menuCompile->Append(ID_COMPILE_EVENT, _("&Compile\tF5"));

    // construct menu
    menuBar->Append (menuFile, _("&File"));
    menuBar->Append (menuEdit, _("&Edit"));
    menuBar->Append (menuView, _("&View"));
    menuBar->Append (menuCompile, _("&Compile"));
    menuBar->Append (menuAnnotations, _("&Annotations"));
    menuBar->Append (menuExtra, _("E&xtra"));
    menuBar->Append (menuWindow, _("&Window"));
    menuBar->Append (menuHelp, _("&Help"));
    SetMenuBar (menuBar);

    menuBar->Check(myID_ANNOTATION_STYLE_BOXED, true);
}
//Open's File to edit
void IDE_CompiladorFrame::FileOpen (wxString fname)
{
    wxFileName w(fname); w.Normalize(); fname = w.GetFullPath();
    editor->LoadFile (fname);
    editor->SelectNone();
}

void IDE_CompiladorFrame::OnCompile(wxCommandEvent& event){
    wxMessageBox(editor->GetFilename(), _("Archivo de compilación"));

    wxString Command(wxT(LEXIC_PATH));
    Command.Append(editor->GetFilename());

    wxArrayString Output, Errors;
    wxArrayString Tokens;

    wxString Out;

    long Code = wxExecute(Command, Output, Errors);

    size_t CountOutput = Output.GetCount();
    wxString Token;
    for(int i = 0; i < CountOutput; ++i){
        //wxMessageBox(Output[i], "Linea");
        Out.Append(Output[i]);
        if(Output[i][Output[i].Length()-1] == ')')
            Out.Append('\n');
    }
    phases->SetText(Out, LEXER_PHASE);
}

IDE_CompiladorFrame::~IDE_CompiladorFrame()
{

}

