#include "loadingwidget.hpp"

LoadingWidget::LoadingWidget(QWidget *parent)
    : QWidget{parent}
{
    layout = new QVBoxLayout(this);

    label = new QLabel("Loading web page...", this);
    label->setAlignment(Qt::AlignmentFlag::AlignCenter);
    label->setFont(QFont("sans-serif", 12));

    layout->addWidget(label);

    this->resize(400, 100);
    this->setWindowFlags(this->windowFlags() | Qt::FramelessWindowHint);
    this->raise();
}
