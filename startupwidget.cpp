#include "startupwidget.hpp"

#include <QFont>
#include <QUrl>

StartupWidget::StartupWidget(const QString &url) : QWidget()
{
    this->_url = resoluteUrl(url);
    this->view = nullptr;
    this->directMode = url.length();

    loadingWidget = new LoadingWidget();

    view = new QWebEngineView();

    // Startup behaviors
    connect(this, &StartupWidget::askForUrl, &StartupWidget::interact);
    connect(this, &StartupWidget::direct, &StartupWidget::launch);
    connect(this, &StartupWidget::loading, loadingWidget, &LoadingWidget::show);

    // Web engine behaviors
    connect(view, &QWebEngineView::titleChanged, &QWebEngineView::setWindowTitle);
    connect(view, &QWebEngineView::iconChanged, &QWebEngineView::setWindowIcon);
    connect(view, &QWebEngineView::loadStarted, loadingWidget, &LoadingWidget::raise);
    connect(view, &QWebEngineView::loadStarted, this, &StartupWidget::close);
    connect(view, &QWebEngineView::loadFinished, loadingWidget, &LoadingWidget::close);
}

void StartupWidget::startup() {
    if (directMode) {
        emit direct(_url);
    }
    else {
        emit askForUrl();
    }
}

void StartupWidget::launch(const QUrl &url) {
    emit loading();

    // Initialize web engine
    view->setUrl(url);
    view->showMaximized();
}

void StartupWidget::interact() {
    this->setWindowTitle("JustShell");

    layout = new QVBoxLayout(this);

    label = new QLabel("Please enter the URL you would like to access", this);
    entry = new QLineEdit(this);

    startupButton = new QPushButton("Start", this);
    startupButton->setEnabled(false);

    layout->addWidget(label);
    layout->addWidget(entry);
    layout->addWidget(startupButton);

    connect(entry, &QLineEdit::textChanged, this, &StartupWidget::updateStartupButtonStatus);
    connect(startupButton, &QPushButton::pressed, this, &StartupWidget::launchWithInput);
    connect(entry, &QLineEdit::returnPressed, startupButton, &QPushButton::click);

    this->resize(350, 160);
    this->show();
}

void StartupWidget::updateStartupButtonStatus() {
    startupButton->setEnabled(entry->text().trimmed().length() > 0);
}

void StartupWidget::launchWithInput() {
    this->launch(resoluteUrl(entry->text()));
}

QUrl StartupWidget::resoluteUrl(const QString &url) {
    QString trimmed = url.trimmed();

    if (patternPrefix.match(trimmed).hasMatch()) {
        return QUrl(trimmed);
    }

    if (patternDomain.match(trimmed.split("/")[0]).hasMatch()) {
        return QUrl("https://" + trimmed);
    }

    return QUrl::fromLocalFile(trimmed);
}
