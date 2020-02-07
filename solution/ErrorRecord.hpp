#include <windows.h>

struct ErrorRecord
{
    PCSTR Message;
    DWORD ErrorCode;
    DWORD LineNumber;
    PCSTR File;
};

void
ReportError(ErrorRecord Rec);