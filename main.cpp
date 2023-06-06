#include <QApplication>

#include "arguments.hpp"
#include "startupwidget.hpp"

void load_webview(const QString &);

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    Arguments args(argc, argv);

    StartupWidget sw(args.path());
    sw.startup();

    return a.exec();
}
