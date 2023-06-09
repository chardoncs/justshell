#ifndef STARTUPWIDGET_HPP
#define STARTUPWIDGET_HPP

#include <QLabel>
#include <QPointer>
#include <QPushButton>
#include <QLineEdit>
#include <QWebEngineView>
#include <QWidget>
#include <QVBoxLayout>

#include "loadingwidget.hpp"

class StartupWidget : public QWidget
{
    Q_OBJECT
public:
    explicit StartupWidget(const QString &path);

public slots:
    void startup();

private:
    QString path_p;
    QPointer<QWebEngineView> view;

    QPointer<LoadingWidget> loadingWidget;

    QPointer<QVBoxLayout> layout;
    QPointer<QLabel> label;
    QPointer<QPushButton> startupButton;
    QPointer<QLineEdit> entry;

    void showLoading();

private slots:
    void launch(const QString &path);
    void interact();

    void updateStartupButtonStatus();

    void launchWithInput();

signals:
    void launchDirectly(const QString &);
    void askForUrl();
    void loading();
};

#endif // STARTUPWIDGET_HPP
