mod juego;
mod models;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use juego::Juego;
use models::Direccion;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Clear, Paragraph,
        canvas::{Canvas, Rectangle},
    },
};

const ANCHO: u16 = 40;
const ALTO: u16 = 20;

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(&mut stdout, EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut juego = Juego::new(ANCHO, ALTO);

    while !juego.exit {
        // 1. Dibujar la interfaz
        terminal.draw(|frame| {
            dibujar_interfaz(frame, &mut juego);
        })?;

        // 2. Procesar entrada del usuario
        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => juego.direccion_serpiente = Direccion::Arriba,
                    KeyCode::Down => juego.direccion_serpiente = Direccion::Abajo,
                    KeyCode::Left => juego.direccion_serpiente = Direccion::Izquierda,
                    KeyCode::Right => juego.direccion_serpiente = Direccion::Derecha,
                    KeyCode::Char('q') => juego.exit = true,
                    _ => {}
                }
            }
        }

        // 3. Actualizar el estado del juego
        juego.mover(ANCHO, ALTO);
        juego.verificar_colisiones(ANCHO, ALTO);
    }

    terminal.draw(|frame| {
        dibujar_interfaz(frame, &mut juego);
    })?;

    loop {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.code == KeyCode::Enter {
                break;
            }
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn dibujar_interfaz(frame: &mut Frame, juego: &mut Juego) {
    // 1. Layout principal
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    // 2. Barra de puntaje
    let puntaje_paragraph = Paragraph::new(format!("Puntaje: {}", juego.puntaje))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Viborita en Rust"),
        );
    frame.render_widget(puntaje_paragraph, chunks[0]);

    // 3. Área de juego
    let area_juego = chunks[1];
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Área de Juego"),
        )
        .x_bounds([0.0, ANCHO as f64])
        .y_bounds([0.0, ALTO as f64])
        .paint(|ctx| {
            // Dibujar comida
            ctx.draw(&Rectangle {
                x: juego.posicion_comida.x as f64,
                y: juego.posicion_comida.y as f64,
                width: 1.0,
                height: 1.0,
                color: juego.color_comida,
            });

            // Dibujar serpiente
            for pos in &juego.serpiente {
                ctx.draw(&Rectangle {
                    x: pos.x as f64,
                    y: pos.y as f64,
                    width: 1.0,
                    height: 1.0,
                    color: Color::Green,
                });
            }
        });

    frame.render_widget(canvas, area_juego);

    if juego.exit {
        let area = centrar_rect(60, 30, frame.area()); // 60% ancho, 30% alto

        let popup = Paragraph::new(format!(
            "¡GAME OVER!\nPuntaje final: {}\nPresiona <Enter> para salir",
            juego.puntaje
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Fin del Juego "),
        )
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .alignment(Alignment::Center);

        frame.render_widget(Clear, area); // Borra lo que hay debajo 🧹
        frame.render_widget(popup, area);
    }
}

// Función para centrar un rectángulo dentro de otro
fn centrar_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
