#include "arguments.hpp"

Arguments::Arguments(int argc, char *argv[]) : QObject()
{
    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-' || argv[i][0] == 0) {
            continue;
        }

        _path = QString(argv[i]);
        break;
    }
}

const QString &Arguments::path() const {
    return _path;
}
