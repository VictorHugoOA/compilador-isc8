#ifndef EDITORTEXT_H
#define EDITORTEXT_H

#include <wx/stc/stc.h>
#include "./../include/prefs.h"
#include <wx/dialog.h>
class EditorText;
class EditProperties;

class EditorText: public wxStyledTextCtrl
{
    friend class EditProperties;
    public:
        EditorText(wxWindow* parent, wxWindowID id = wxID_ANY, const wxPoint &pos = wxDefaultPosition, const wxSize &size = wxDefaultSize,
                   long style = wxVSCROLL|
                   #ifndef __WXMAC__
                   wxSUNKEN_BORDER
                   #endif // __WXMAC__
                   );
        virtual ~EditorText();

        //Event handlers
        //Resize window
        void OnSize(wxSizeEvent &event);
        //edit
        void OnEditRedo(wxCommandEvent &event);
        void OnEditUndo(wxCommandEvent &event);
        void OnEditClear(wxCommandEvent &event);
        void OnEditCut(wxCommandEvent &event);
        void OnEditCopy(wxCommandEvent &event);
        void OnEditPaste(wxCommandEvent &event);
        //find events
        void OnFind(wxCommandEvent &event);
        void OnFindNext(wxCommandEvent &event);
        void OnReplace(wxCommandEvent &event);
        void OnReplaceNext(wxCommandEvent &event);
        void OnBraceMatch(wxCommandEvent &event);
        void OnGoto(wxCommandEvent &event);
        void OnEditIndentInc(wxCommandEvent &event);
        void OnEditIndentRed(wxCommandEvent &event);
        void OnEditSelectAll(wxCommandEvent &event);
        void OnEditSelectLine(wxCommandEvent &event);
        //view
        void OnHilightLang(wxCommandEvent &event);
        void OnDisplayEOL(wxCommandEvent &event);
        void OnIndentGuide(wxCommandEvent &event);
        void OnLineNumber(wxCommandEvent &event);
        void OnLongLineOn(wxCommandEvent &event);
        void OnWhiteSpace(wxCommandEvent &event);
        void OnFoldToggle(wxCommandEvent &event);
        void OnSetOverType(wxCommandEvent &event);
        void OnSetReadOnly(wxCommandEvent &event);
        void OnWrapmodeOn(wxCommandEvent &event);
        void OnUseCharset(wxCommandEvent &event);
        //annotations
        void OnAnnotationAdd(wxCommandEvent &event);
        void OnAnnotationRemove(wxCommandEvent &event);
        void OnAnnotationClear(wxCommandEvent &event);
        void OnAnnotationStyle(wxCommandEvent &event);
        //Extra
        void OnChangeCase(wxCommandEvent &event);
        void OnConvertEOL(wxCommandEvent &event);
        //stc
        void OnMarginClick(wxStyledTextEvent &event);
        void OnCharAdded(wxStyledTextEvent &event);
        void OnKey(wxStyledTextEvent &event);
        void OnKeyDown(wxKeyEvent &event);

        //language/lexer
        wxString DeterminePrefs(const wxString &filename);
        bool InitializePrefs(const wxString &filename);
        bool UserSettings(const wxString &filename);
        LanguageInfo const* GetLanguageInfo(){return language;}

        //Get file name and set file name
        wxString GetFilename(){return fileName;}
        void SetFilename(const wxString &str){this->fileName = str;}

        //file manipulation
        bool LoadFile();
        bool LoadFile(const wxString &file);
        bool SaveFile();
        bool SaveFile(const wxString &file);
        bool Modified();
        void HighlightSyntax(size_t From, size_t To, wxString &Text);

    protected:

    private:
        wxString fileName; //filename editing
        LanguageInfo const* language;


        //margin variables
        int lineNrMargin;
        int lineNrID;
        int foldingID;
        int foldingMargin;
        int dividerID;

        wxDECLARE_EVENT_TABLE();
};

class EditProperties: public wxDialog{

public:
    EditProperties(EditorText *editor, long style = 0);

};



#endif // EDITORTEXT_H
