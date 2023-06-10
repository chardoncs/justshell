#ifndef LOADINGWIDGET_HPP
#define LOADINGWIDGET_HPP

#include <QLabel>
#include <QPointer>
#include <QVBoxLayout>
#include <QWidget>

class LoadingWidget : public QWidget
{
    Q_OBJECT
public:
    explicit LoadingWidget(QWidget *parent = nullptr);

private:
    QPointer<QVBoxLayout> layout;

    QPointer<QLabel> label;

signals:

};

#endif // LOADINGWIDGET_HPP
