#ifndef ARGUMENTS_HPP
#define ARGUMENTS_HPP

#include <QObject>
#include <QRegularExpression>
#include <QUrl>

class Arguments : public QObject
{
    Q_OBJECT
public:
    explicit Arguments(int argc, char *argv[]);

    const QString &url() const;

private:
    QString url_str;

signals:

};

#endif // ARGUMENTS_HPP
