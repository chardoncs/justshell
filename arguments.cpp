#include "arguments.hpp"

Arguments::Arguments(int argc, char *argv[]) : QObject()
{
    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-' || argv[i][0] == 0) {
            continue;
        }

        url_str = QString(argv[i]);
        break;
    }
}

const QString &Arguments::url() const {
    return url_str;
}
