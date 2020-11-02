import 'dart:io';

main() {
  var f = File('misc/fix.rs');
  String s = f.readAsStringSync();

  s = s.replaceAllMapped(new RegExp(r'self\.dispatch\[(.+)\] = \|cpu: &mut MCS51\| {'), (Match m) => '${m[1]} => {');
  s = s.replaceAll('cpu.', 'self.');
  s = s.replaceAll('};', '},');

  var ff = File('misc/fixed.rs');
  ff.writeAsStringSync(s);
}