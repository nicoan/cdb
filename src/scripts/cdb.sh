cdb () {
  case $1 in
    -*)
      ${CDB_MANAGER_PATH} $@
    ;;
    *)
      result=$(${CDB_MANAGER_PATH} $@)
      instruction=$(echo $result | awk '{print $1}')
      path=$(echo $result | awk '{print $2}')
      if [ "$instruction" != "" ] && [ "$instruction" == "CDB" ]; then
        cd $path
      fi
  esac
}

