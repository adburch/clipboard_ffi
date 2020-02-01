#include <iostream>
#include <vector>
#include <string>

#include <windows.h>


using namespace std;

int wmain(int argc, const wchar_t** argv)
{
    auto success = OpenClipboard(nullptr);
    if (!success)
    {
       auto error = GetLastError();
       wprintf_s(L"Failed to open clipboard: %x", error);
       return error;
    }

    if (!IsClipboardFormatAvailable(CF_UNICODETEXT))
    {
        auto error = GetLastError();
        wprintf_s(L"No text on clipboard: %x", error);
        return error;
    }

    auto hglb = GetClipboardData(CF_UNICODETEXT);
    if (hglb != NULL)
    {
        auto lpwstr = GlobalLock(hglb);
        if (lpwstr != NULL)
        {
            // Call the application-defined ReplaceSelection
            // function to insert the text and repaint the
            // window.

            wprintf_s(L"Clipboard data: %wS", lpwstr);
            GlobalUnlock(hglb);
        }
    }
    CloseClipboard();
}
