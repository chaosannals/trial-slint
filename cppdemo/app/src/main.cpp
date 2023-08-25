#include "app-window.h"
#if WIN32
#include <Windows.h>
#endif


int main(int argc, char **argv)
{
#if WIN32
    // TODO ����̫���ˣ�������������
    ShowWindow(GetConsoleWindow(), SW_HIDE);
#endif

    auto ui = AppWindow::create();

    ui->on_request_increase_value([&]{
        ui->set_counter(ui->get_counter() + 1);
    });

    ui->run();
    return 0;
}
