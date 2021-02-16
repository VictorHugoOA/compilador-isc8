#ifndef TEXTEDITOR_H
#define TEXTEDITOR_H

#include <wx/stc/stc.h>

class TextEditor: public wxStyledTextCtrl{

public:
    TextEditor(wxWindow *parent);
    ~TextEditor();
};

#endif // TEXTEDITOR_H
