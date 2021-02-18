#include "ResultsShow.h"
#include "EditorText.h"

ResultsShow::ResultsShow(wxWindow *parent, wxWindowID id, const wxPoint &pos, const wxSize& size): wxNotebook(parent, id, pos, size, wxNB_TOP|wxNB_FIXEDWIDTH)
{
    //Create panels
    results = new EditorText(this);
    error = new EditorText(this);
    error->SetReadOnly(false);
    error->SetText("Here goes all error text");
    error->SetReadOnly(true);
    results->SetReadOnly(false);
    results->SetText("Here goes all results text");
    results->SetReadOnly(true);
    //Insert panels at index with title
    this->InsertPage(0, results, "Results", true);//true means this is the focused one
    this->InsertPage(1, error, "Error", false);
}

ResultsShow::~ResultsShow()
{
    //dtor
}
