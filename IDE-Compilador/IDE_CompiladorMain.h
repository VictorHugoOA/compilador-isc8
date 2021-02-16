/***************************************************************
 * Name:      IDE_CompiladorMain.h
 * Purpose:   Defines Application Frame
 * Author:     ()
 * Created:   2021-02-08
 * Copyright:  ()
 * License:
 **************************************************************/

#ifndef IDE_COMPILADORMAIN_H
#define IDE_COMPILADORMAIN_H

#include <wx/frame.h>
#include <wx/wx.h>
//! wxWidgets headers
#include "wx/config.h"   // configuration support
#include "wx/filedlg.h"  // file dialog support
#include "wx/filename.h" // filename support
#include "wx/notebook.h" // notebook support
#include "wx/settings.h" // system settings
#include "wx/string.h"   // strings support
#include "wx/image.h"    // images support

#include "EditorText.h"

#include "IDE_CompiladorApp.h"

class IDE_CompiladorApp;

enum{
    ID_HELLO = 1
};

class IDE_CompiladorFrame: public wxFrame
{
    friend class IDE_CompiladorApp;

    public:

        IDE_CompiladorFrame(wxWindow* parent,wxWindowID id = -1);
        virtual ~IDE_CompiladorFrame();


        void OnClose (wxCloseEvent &event);
        void OnAbout(wxCommandEvent& event);
        void OnResize(wxSizeEvent& event);
        void OnTimerEvent (wxTimerEvent &event);
          //! file
        void OnFileNew (wxCommandEvent &event);
        void OnFileNewFrame (wxCommandEvent &event);
        void OnFileOpen (wxCommandEvent &event);
        void OnFileOpenFrame (wxCommandEvent &event);
        void OnFileSave (wxCommandEvent &event);
        void OnFileSaveAs (wxCommandEvent &event);
        void OnFileClose (wxCommandEvent &event);
        //! properties
        void OnProperties (wxCommandEvent &event);
        //! edit events
        void OnEdit (wxCommandEvent &event);
        void OnExit (wxCommandEvent &event);

    private:
        EditorText* editor; //The code editor.
        void FileOpen(wxString fname);

        wxMenuBar *menuBar;
        void CreateMenu();

        DECLARE_EVENT_TABLE();
};

#endif // IDE_COMPILADORMAIN_H
