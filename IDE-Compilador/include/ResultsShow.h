#ifndef RESULTSSHOW_
#define RESULTSSHOW_

#include <wx/notebook.h>
#include <wx/panel.h>
class EditorText;

class ResultsShow: public wxNotebook
{
    public:
        ResultsShow(wxWindow *parent, wxWindowID id = wxID_ANY, const wxPoint &pos = wxDefaultPosition, const wxSize &size = wxDefaultSize);
        virtual ~ResultsShow();

    protected:

    private:
        //Panel methods to handle text WIP
        EditorText* error;
        EditorText* results;
};

#endif // RESULTSSHOW_
