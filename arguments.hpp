#ifndef ARGUMENTS_HPP
#define ARGUMENTS_HPP

#include <QObject>

class Arguments : public QObject
{
    Q_OBJECT
public:
    explicit Arguments(int argc, char *argv[]);

    const QString &path() const;

private:
    QString _path;

signals:

};

#endif // ARGUMENTS_HPP
