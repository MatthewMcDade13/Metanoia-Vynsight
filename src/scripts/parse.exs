defmodule MetaVyn.Parser do

  @spec ls_code_to_base64(String.t(), [String.t()]) :: [String.t()]
  def ls_code_to_base64(dir_path, file_exts) when is_list(file_exts) do
    ls_code_to_base64(dir_path, MapSet.new(file_exts))
  end

  @spec ls_code_to_base64(String.t(), MapSet.t()) :: [String.t()]
  def ls_code_to_base64(dir_path, file_exts) when is_map(file_exts) do
    dir_path
      |> File.ls!
      |> Enum.filter(fn x -> MapSet.member?(file_exts, Path.extname x) end)
      |> Enum.map(fn x ->
          Path.join(dir_path, x)
            |> File.read!
            |> :zlib.gzip
            |> Base.encode64
      end)
  end

end

