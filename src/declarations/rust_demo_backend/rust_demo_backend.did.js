export const idlFactory = ({ IDL }) => {
  const CommitBatch = IDL.Record({
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'chunkIds' : IDL.Vec(IDL.Nat),
    'batchId' : IDL.Nat,
  });
  const AssetKey = IDL.Record({
    'token' : IDL.Opt(IDL.Text),
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'name' : IDL.Text,
    'fullPath' : IDL.Text,
    'folder' : IDL.Text,
  });
  const InitUpload = IDL.Record({ 'batchId' : IDL.Nat });
  const Chunk = IDL.Record({
    'content' : IDL.Vec(IDL.Nat8),
    'batchId' : IDL.Nat,
  });
  const UploadChunk = IDL.Record({ 'chunkId' : IDL.Nat });
  return IDL.Service({
    'commitUpload' : IDL.Func([CommitBatch], [], []),
    'initUpload' : IDL.Func([AssetKey], [InitUpload], []),
    'transferFreezingThresholdCycles' : IDL.Func([], [], []),
    'uploadChunk' : IDL.Func([Chunk], [UploadChunk], []),
  });
};
export const init = ({ IDL }) => { return []; };
