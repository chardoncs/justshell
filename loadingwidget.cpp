#include "loadingwidget.hpp"

#include <QLabel>

LoadingWidget::LoadingWidget(QWidget *parent)
    : QWidget{parent}
{
    QLabel *label = new QLabel("Initializing browser window...", this);
    label->setAlignment(Qt::AlignmentFlag::AlignCenter);
    label->setFont(QFont("sans-serif", 16));

    this->resize(400, 100);
    this->setWindowFlags(this->windowFlags() | Qt::FramelessWindowHint);
}
