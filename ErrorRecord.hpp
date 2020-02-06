#include <windows.h>

struct ErrorRecord
{
    PCWSTR Message;
    DWORD ErrorCode;
    DWORD LineNumber;
    PSTR Function;
};

void
ReportError(ErrorRecord Rec);