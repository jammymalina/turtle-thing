unit RobotUnit;
// verzia 221012
{$mode objfpc}{$H+}

interface

uses
  Classes, SysUtils,  Forms, Controls, Graphics, Dialogs,  ExtCtrls;

type
  TRobot = class
  private
    FX, FY, FH: Real;
    FDown: Boolean;
    FPC: TColor;
    FPW: Integer;
  public
    Value: Integer;
    constructor Create;
    constructor Create(NewX, NewY: Real; Angle: Real = 0);

    procedure Fd(D: Real); virtual;
    procedure Rt(Angle: Real); virtual;
    procedure Lt(Angle: Real); virtual;
    procedure SetH(Angle: Real); virtual;
    procedure SetXY(NewX, NewY: Real); virtual;
    procedure SetX(NewX: Real); virtual;
    procedure SetY(NewY: Real); virtual;
    procedure MoveXY(NewX, NewY: Real); virtual;
    procedure PU; virtual;
    procedure PD; virtual;
    procedure SetPen(Down: Boolean); virtual;
    procedure SetPC(NewColor: TColor); virtual;
    procedure SetPW(NewWidth: Integer); virtual;

    procedure Point(NewWidth: Real = 0); virtual;
    procedure Text(NewText: string); virtual;
    procedure Towards(AnyX, AnyY: Real); virtual;
    procedure Fill(NewColor: TColor); virtual;
    function  Dist(AnyX, AnyY: Real): Real; virtual;
    function  IsNear(AnyX, AnyY: Real): Boolean; virtual;
    procedure Draw; virtual;

    property  X: Real read FX write SetX;
    property  Y: Real read FY write SetY;
    property  H: Real read FH write SetH;
    property  IsPD: Boolean read FDown write SetPen;
    property  PC: TColor read FPC write SetPC;
    property  PW: Integer read FPW write SetPW;
  end;

//////////////////////////////////////////////////////////////////////////////////////////

const
  Rad = Pi/180;
  Deg = 180/Pi;

procedure CS(NewColor: TColor = clWhite);
procedure Wait(MilliSec: Integer);

procedure SetImage(Image: TImage = nil);

//////////////////////////////////////////////////////////////////////////////////////////

implementation

var
  MyImage: TImage = nil;

//////////////////////////////////////////////////////////////////////////////////////////

constructor TRobot.Create;
begin
  SetImage;
  Create(MyImage.Width / 2, MyImage.Height / 2);
end;

constructor TRobot.Create(NewX, NewY, Angle: Real);
begin
  SetImage;
  FX := NewX;
  FY := NewY;
  FH := Angle;
  FPC := clBlack;
  FPW := 1;
  FDown := True;
end;

procedure TRobot.Rt(Angle: Real);
begin
  SetH(FH + Angle);
end;

procedure TRobot.Lt(Angle: Real);
begin
  SetH(FH - Angle);
end;

procedure TRobot.SetH(Angle: Real);
begin
  FH := Angle;
  while FH < 0 do
    FH := FH + 360;
  while FH >= 360 do
    FH := FH - 360;
end;

procedure TRobot.Fd(D: Real);
begin
  SetXY(FX + Sin(FH * Rad) * D, FY - Cos(FH * Rad) * D);
end;

procedure TRobot.SetXY(NewX, NewY: Real);
begin
  if not FDown then
    MoveXY(NewX, NewY)
  else
    with MyImage.Canvas do
    begin
      Brush.Style := bsSolid;
      Pen.Color := FPC;
      Pen.Width := FPW;
      Line(Round(FX), Round(FY), Round(NewX), Round(NewY));
      FX := NewX;
      FY := NewY;
    end;
end;

procedure TRobot.MoveXY(NewX, NewY: Real);
begin
  FX := NewX;
  FY := NewY;
  MyImage.Canvas.MoveTo(Round(FX), Round(FY));
end;

procedure TRobot.PU;
begin
  FDown := False;
end;

procedure TRobot.PD;
begin
  FDown := True;
end;

procedure TRobot.SetPen(Down: Boolean);
begin
  if Down then
    PD
  else
    PU;
end;

procedure TRobot.SetPC(NewColor: TColor);
begin
  FPC := NewColor;
end;

procedure TRobot.SetPW(NewWidth: Integer);
begin
  FPW := NewWidth;
end;

procedure TRobot.SetX(NewX: Real);
begin
  SetXY(NewX, FY);
end;

procedure TRobot.SetY(NewY: Real);
begin
  SetXY(FX, NewY);
end;

//////////////////////////////////////////////////////////////////////////////////////////

procedure TRobot.Point(NewWidth: Real);
var
  AnyX, AnyY, R1, R2: Integer;
begin
  NewWidth := Round(Abs(NewWidth));
  if NewWidth = 0 then
    NewWidth := FPW;
  R1 := (Trunc(NewWidth) + 1) div 2;
  R2 := Trunc(NewWidth) - R1 + 1;
  with MyImage.Canvas do
  begin
    AnyX := Round(FX);
    AnyY := Round(FY);
    Brush.Color := FPC;
    Brush.Style := bsSolid;
    Pen.Color := FPC;
    Pen.Style := psSolid;
    Pen.Width := 1;
    Ellipse(AnyX - R1, AnyY - R1, AnyX + R2, AnyY + R2);
  end;
end;

procedure TRobot.Fill(NewColor: TColor);
var
  AnyX, AnyY: Integer;
begin
  with MyImage.Canvas do
  begin
    AnyX := Round(FX);
    AnyY := Round(FY);
    Brush.Color := NewColor;
    Brush.Style := bsSolid;
    FloodFill(AnyX, AnyY, Pixels[AnyX, AnyY], fsSurface);
  end;
end;

procedure TRobot.Towards(AnyX, AnyY: Real);
var
  Angle: Real;
begin
  AnyX := AnyX - FX;
  AnyY := FY - AnyY;
  if AnyY = 0 then
    if AnyX = 0 then
      Angle := 0
    else if AnyX < 0 then
      Angle := 270
    else
      Angle := 90
  else if AnyY > 0 then
    if AnyX >= 0 then
      Angle := ArcTan(AnyX / AnyY) * Deg
    else
      Angle := 360 - ArcTan(- AnyX / AnyY) * Deg
  else if AnyX >= 0 then
    Angle := 180 - ArcTan(- AnyX / AnyY) * Deg
  else
    Angle := 180 + ArcTan(AnyX / AnyY) * Deg;
  SetH(Angle);
end;

function TRobot.Dist(AnyX, AnyY: Real): Real;
begin
  Result := Sqrt(Sqr(AnyX - FX) + Sqr(AnyY - FY));
end;

function TRobot.IsNear(AnyX, AnyY: Real): Boolean;
begin
  Result := Sqr(AnyX - FX) + Sqr(AnyY - FY) < 100;
end;

procedure TRobot.Text(NewText: string);
begin
  MyImage.Canvas.Font.Color := FPC;
  MyImage.Canvas.Brush.Style := bsClear;
  MyImage.Canvas.TextOut(Round(FX), Round(FY), NewText);
end;

procedure TRobot.Draw;
begin
  Point;
end;

//////////////////////////////////////////////////////////////////////////////////////////

procedure CS(NewColor: TColor);
begin
  SetImage;
  with MyImage, Canvas do
  begin
    Brush.Color := NewColor;
    Brush.Style := bsSolid;
    FillRect(ClientRect);
  end;
end;

procedure Wait(MilliSec: Integer);
begin
  if MyImage <> nil then
    MyImage.Parent.Repaint;
  Sleep(MilliSec);
end;

//////////////////////////////////////////////////////////////////////////////////////

procedure SetImage(Image: TImage);
var
  I, N: Integer;
  Form: TForm;
begin
  if MyImage <> nil then
    Exit;
  if Image = nil then
  begin
    Form := Application.MainForm;
    if Form = nil then
    begin
      N := Application.ComponentCount;
      I := 0;
      while (I < N) and not (Application.Components[I] is TForm) do
        Inc(I);
      if I = N then
      begin
        ShowMessage('vadnÃ¡ aplikÃ¡cia - Robot nenaÅ¡iel formulÃ¡r');
        Halt;
      end;
      Form := TForm(Application.Components[I]);
    end;
    N := Form.ControlCount;
    I := 0;
    while (I < N) and not (Form.Controls[I] is TImage) do
      Inc(I);
    if I >= N then
    begin
      ShowMessage('vadnÃ¡ aplikÃ¡cia - Robot nenaÅ¡iel grafickÃº plochu');
      Halt;
    end;
    Image := TImage(Form.Controls[I]);
  end;
  with Image, Canvas do
  begin
    Brush.Color := clWhite;
    Brush.Style := bsSolid;
    FillRect(ClientRect);
  end;
  if Image.Owner is TForm then
    TForm(Image.Owner).DoubleBuffered := True;
  MyImage := Image;
end;

end.
