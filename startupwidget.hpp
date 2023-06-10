#ifndef STARTUPWIDGET_HPP
#define STARTUPWIDGET_HPP

#include <QLabel>
#include <QPointer>
#include <QPushButton>
#include <QLineEdit>
#include <QRegularExpression>
#include <QWebEngineView>
#include <QWidget>
#include <QVBoxLayout>

#include "loadingwidget.hpp"

class StartupWidget : public QWidget
{
    Q_OBJECT
public:
    explicit StartupWidget(const QString &);

public slots:
    void startup();

private:
    QUrl _url;
    bool directMode;

    QPointer<QWebEngineView> view;

    QPointer<LoadingWidget> loadingWidget;

    QPointer<QVBoxLayout> layout;
    QPointer<QLabel> label;
    QPointer<QPushButton> startupButton;
    QPointer<QLineEdit> entry;

    void showLoading();

    QRegularExpression patternPrefix{"^(https?|ftp)://"};
    QRegularExpression patternDomain{"^([\\d\\w]+\\.)+[\\d\\w]+$"};

    QUrl resoluteUrl(const QString &);

private slots:
    void launch(const QUrl &);
    void interact();

    void updateStartupButtonStatus();

    void launchWithInput();

signals:
    void direct(const QUrl &);
    void askForUrl();
    void loading();
};

#endif // STARTUPWIDGET_HPP
