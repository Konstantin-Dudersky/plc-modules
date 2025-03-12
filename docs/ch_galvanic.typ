= Гальваническая изоляция

Гальваническая изоляция в модулях ввода / вывода применяется по разным причинам:

- Предотвращает протекание выравнивающего тока из-за разности потенциалов объектов, заземленных на разных шинах @SLLA529. У разных модулей и контроллера независимое подключение к датчикам / исполнительным механизмам.

- Повышает устойчивость к разрядам статического электричества (ESD), кратковременным выбросам повышенного напряжения (EFT) @SLYT725.

TODO - Нарисовать схему с областями изоляции.

== Применяемые компоненты для обеспечения гальванической изоляции

=== Изолированные DC / DC преобразователи

=== Преобразователи интерфейса RS-485 со встроенной изоляцией <isolation_rs_485>


=== Изоляция дискретных сигналов высокого напряжения <isolation_di>

В промышленности для подключения цифровых датчиков наиболее часто применяется постоянное напряжение 24 В, тогда как микроконтроллеры работают, как правило, на напряжении 3,3 В или 5 В.

В данном проекте в качестве изолятора применяется чип CA-IS3980S @CA-IS3980S_datasheet. Чип обеспечивает изоляцию 2500 $V_("RMS")$ и $±300 "kV"/"µs"$ CMTI. Фунциональная схема показана на @CA-IS3980S[рисунке].

#figure(
  caption: "Функциональная схема изолятора цифровых входов",
  image("images/CA-IS3980P.png", height: 200pt),
) <CA-IS3980S>

На @CA-IS3980S_channel[рисунке] показана упрощенная схема отдельного канала. The internal LED emulator output drives an ON-OFF keying (OOK) modulator, to transfer digital signals across the SiO2 based isolation barrier between circuits with different power domains. In many applications, this capacitive isolation technology is replacing optocoupler-based solution because it can reduce the power requirements and take up less board space while offering the same isolation capability.

#figure(
  caption: "Упрощенная схема для отдельного канала",
  image("images/CA-IS3980P_channel.png", width: 100%),
) <CA-IS3980S_channel>

On the output side, the signal is either passed directly to the output stage in the case of a high-speed channel (BHx), or the signal is routed through a debounce filter block in the case of a low-speed channel (Bx) for robust operation in industrial environments.

Данный чип применен на плате #link(<PM_CNV-DI16_sink>)[PM_CNV-DI16_sink].

=== Изоляция цифровых сигналов
