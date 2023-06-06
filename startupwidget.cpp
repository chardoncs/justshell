#include "startupwidget.hpp"

#include <QFont>
#include <QUrl>

StartupWidget::StartupWidget(const QString &path) : QWidget()
{
    this->path_p = path.trimmed();
    this->view = nullptr;

    loadingWidget = new LoadingWidget();

    view = new QWebEngineView();

    // Startup behaviors
    connect(this, &StartupWidget::askForUrl, &StartupWidget::interact);
    connect(this, &StartupWidget::launchDirectly, &StartupWidget::launch);
    connect(this, &StartupWidget::loading, &StartupWidget::initializeLoadingWindow);
    // Web engine behaviors
    connect(view, &QWebEngineView::titleChanged, &QWebEngineView::setWindowTitle);
    connect(view, &QWebEngineView::iconChanged, &QWebEngineView::setWindowIcon);
    connect(view, &QWebEngineView::loadStarted, loadingWidget, &LoadingWidget::raise);
    connect(view, &QWebEngineView::loadStarted, this, &StartupWidget::close);
    connect(view, &QWebEngineView::loadFinished, loadingWidget, &LoadingWidget::close);
}

void StartupWidget::startup() {
    if (path_p.length() == 0) {
        emit askForUrl();
    }
    else {
        emit launchDirectly(path_p);
    }
}

void StartupWidget::launch(const QString &path) {
    emit loading();

    // Initialize web engine
    view->setUrl(QUrl(path));
    view->showMaximized();
}

void StartupWidget::interact() {
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

void StartupWidget::initializeLoadingWindow() {
    loadingWidget->show();
}

void StartupWidget::updateStartupButtonStatus() {
    startupButton->setEnabled(entry->text().trimmed().length() > 0);
}

void StartupWidget::launchWithInput() {
    this->launch(entry->text().trimmed());
}
