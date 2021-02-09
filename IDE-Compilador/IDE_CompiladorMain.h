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

//(*Headers(IDE_CompiladorFrame)
#include <wx/frame.h>
#include <wx/menu.h>
#include <wx/statusbr.h>
//*)

class IDE_CompiladorFrame: public wxFrame
{
    public:

        IDE_CompiladorFrame(wxWindow* parent,wxWindowID id = -1);
        virtual ~IDE_CompiladorFrame();

    private:

        //(*Handlers(IDE_CompiladorFrame)
        void OnQuit(wxCommandEvent& event);
        void OnAbout(wxCommandEvent& event);
        //*)

        //(*Identifiers(IDE_CompiladorFrame)
        static const long idMenuQuit;
        static const long idMenuAbout;
        static const long ID_STATUSBAR1;
        //*)

        //(*Declarations(IDE_CompiladorFrame)
        wxStatusBar* StatusBar1;
        //*)

        DECLARE_EVENT_TABLE()
};

#endif // IDE_COMPILADORMAIN_H
