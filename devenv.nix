{
  pkgs,
  ...
}:

{
  packages = with pkgs; [
    bacon
    git
  ];

  languages.rust.enable = true;
}
